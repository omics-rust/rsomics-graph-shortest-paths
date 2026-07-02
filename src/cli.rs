use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser};
use serde::Serialize;

use rsomics_common::{CommonFlags, Result, RsomicsError, ToolMeta, run};

use rsomics_graph_shortest_paths::{bfs, io};

pub const META: ToolMeta = ToolMeta {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
};

/// Metric to compute.
#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false)]
pub struct MetricFlags {
    /// Maximum eccentricity (longest shortest path). Errors if disconnected.
    #[arg(long)]
    pub diameter: bool,

    /// Average shortest path length Σ d(u,v)/(n(n-1)). Errors if disconnected.
    #[arg(long)]
    pub average: bool,

    /// Single-source BFS distances from NODE (node TAB dist table).
    #[arg(long, value_name = "NODE")]
    pub source: Option<String>,

    /// Per-node maximum distance (node TAB eccentricity).
    #[arg(long)]
    pub eccentricity: bool,

    /// Minimum eccentricity. Errors if disconnected.
    #[arg(long)]
    pub radius: bool,

    /// Nodes with eccentricity equal to the radius (one label per line).
    #[arg(long)]
    pub center: bool,

    /// Nodes with eccentricity equal to the diameter (one label per line).
    #[arg(long)]
    pub periphery: bool,

    /// Nodes minimising total distance sum Σ_u d(v,u) (one label per line).
    #[arg(long)]
    pub barycenter: bool,
}

/// Shortest-path metrics for undirected graphs.
///
/// Reads an edge list (one `u v` per line, whitespace-separated) from a file
/// argument or stdin (`-`). Comment lines starting with `#` and blank lines
/// are ignored. A self-loop registers its node but adds no distance edge (a
/// self-loop-only node stays isolated); duplicate edges collapse to a simple
/// graph. Every node appearing on any line exists in the graph.
///
/// BFS integer distances are exact. `--average` uses one IEEE-754 division on
/// the integer sum — bit-exact with networkx. `--diameter`, `--radius`, and
/// `--eccentricity` are integer-exact. `--center`, `--periphery`, and
/// `--barycenter` are value-exact with networkx 3.6.1.
#[derive(Parser, Debug)]
#[command(name = "rsomics-graph-shortest-paths", version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub metric: MetricFlags,

    /// Edge list file (`-` or omitted reads stdin).
    #[arg(value_name = "EDGELIST")]
    pub edgelist: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonFlags,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Out {
    Diameter { diameter: u32 },
    Radius { radius: u32 },
    Average { average: f64 },
    Source { distances: Vec<SourceRow> },
    Eccentricity { eccentricities: Vec<EccRow> },
    Center { center: Vec<String> },
    Periphery { periphery: Vec<String> },
    Barycenter { barycenter: Vec<String> },
}

#[derive(Serialize)]
struct SourceRow {
    node: String,
    distance: u32,
}

#[derive(Serialize)]
struct EccRow {
    node: String,
    eccentricity: u32,
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let common = self.common.clone();
        run(&common, META, || self.execute(&common))
    }

    fn execute(self, common: &CommonFlags) -> Result<Out> {
        let path = self.edgelist.as_deref();
        let g = io::read_edgelist(path)?;

        if self.metric.diameter {
            let d = bfs::diameter(&g).map_err(RsomicsError::InvalidInput)?;
            if !common.json {
                println!("{d}");
            }
            return Ok(Out::Diameter { diameter: d });
        }

        if self.metric.radius {
            let r = bfs::radius(&g).map_err(RsomicsError::InvalidInput)?;
            if !common.json {
                println!("{r}");
            }
            return Ok(Out::Radius { radius: r });
        }

        if self.metric.average {
            let a = bfs::average_shortest_path_length(&g).map_err(RsomicsError::InvalidInput)?;
            if !common.json {
                println!("{a}");
            }
            return Ok(Out::Average { average: a });
        }

        if let Some(ref node_label) = self.metric.source {
            let src_id = g
                .labels
                .iter()
                .position(|l| l == node_label)
                .ok_or_else(|| {
                    RsomicsError::InvalidInput(format!(
                        "source node '{node_label}' not found in graph"
                    ))
                })?;
            let pairs = bfs::single_source(&g, src_id as u32);
            let rows: Vec<SourceRow> = pairs
                .iter()
                .map(|&(nid, dist)| SourceRow {
                    node: g.labels[nid as usize].clone(),
                    distance: dist,
                })
                .collect();
            if !common.json {
                let stdout = std::io::stdout().lock();
                let mut w = BufWriter::new(stdout);
                for r in &rows {
                    writeln!(w, "{}\t{}", r.node, r.distance).map_err(RsomicsError::Io)?;
                }
                w.flush().map_err(RsomicsError::Io)?;
            }
            return Ok(Out::Source { distances: rows });
        }

        if self.metric.center {
            let ids = bfs::center(&g).map_err(RsomicsError::InvalidInput)?;
            let labels: Vec<String> = ids.iter().map(|&i| g.labels[i as usize].clone()).collect();
            if !common.json {
                let stdout = std::io::stdout().lock();
                let mut w = BufWriter::new(stdout);
                for l in &labels {
                    writeln!(w, "{l}").map_err(RsomicsError::Io)?;
                }
                w.flush().map_err(RsomicsError::Io)?;
            }
            return Ok(Out::Center { center: labels });
        }

        if self.metric.periphery {
            let ids = bfs::periphery(&g).map_err(RsomicsError::InvalidInput)?;
            let labels: Vec<String> = ids.iter().map(|&i| g.labels[i as usize].clone()).collect();
            if !common.json {
                let stdout = std::io::stdout().lock();
                let mut w = BufWriter::new(stdout);
                for l in &labels {
                    writeln!(w, "{l}").map_err(RsomicsError::Io)?;
                }
                w.flush().map_err(RsomicsError::Io)?;
            }
            return Ok(Out::Periphery { periphery: labels });
        }

        if self.metric.barycenter {
            let ids = bfs::barycenter(&g).map_err(RsomicsError::InvalidInput)?;
            let labels: Vec<String> = ids.iter().map(|&i| g.labels[i as usize].clone()).collect();
            if !common.json {
                let stdout = std::io::stdout().lock();
                let mut w = BufWriter::new(stdout);
                for l in &labels {
                    writeln!(w, "{l}").map_err(RsomicsError::Io)?;
                }
                w.flush().map_err(RsomicsError::Io)?;
            }
            return Ok(Out::Barycenter { barycenter: labels });
        }

        // eccentricity
        if g.n() == 0 {
            return Err(RsomicsError::InvalidInput(
                "eccentricity is undefined for the null graph".into(),
            ));
        }
        if !bfs::is_connected(&g) {
            return Err(RsomicsError::InvalidInput(
                "Found infinite path length because the graph is not connected".into(),
            ));
        }
        let ecc = bfs::eccentricities(&g);
        let rows: Vec<EccRow> = g
            .labels
            .iter()
            .zip(ecc.iter())
            .map(|(label, &e)| EccRow {
                node: label.clone(),
                eccentricity: e,
            })
            .collect();
        if !common.json {
            let stdout = std::io::stdout().lock();
            let mut w = BufWriter::new(stdout);
            for r in &rows {
                writeln!(w, "{}\t{}", r.node, r.eccentricity).map_err(RsomicsError::Io)?;
            }
            w.flush().map_err(RsomicsError::Io)?;
        }
        Ok(Out::Eccentricity {
            eccentricities: rows,
        })
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        super::Cli::command().debug_assert();
    }
}

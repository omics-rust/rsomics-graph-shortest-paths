use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rsomics_common::{Result, RsomicsError};

/// Adjacency structure with integer-mapped node IDs in insertion order.
pub struct Graph {
    /// Node labels in insertion order.
    pub labels: Vec<String>,
    /// Sorted, deduplicated neighbour lists (no self-loops).
    pub adj: Vec<Vec<u32>>,
}

impl Graph {
    pub fn n(&self) -> usize {
        self.labels.len()
    }
}

/// Parse an undirected edge list, matching `nx.read_edgelist` conventions.
///
/// - Lines starting with `#` are comments (ignored).
/// - Blank lines are ignored.
/// - Each data line needs at least two whitespace-separated tokens; extras ignored.
/// - A self-loop (`u u`) registers node `u` but adds no distance edge — a node
///   reachable only via a self-loop stays isolated, exactly like `nx.read_edgelist`.
/// - Duplicate edges collapse to a simple graph.
/// - Every node appearing on any line exists in the graph.
pub fn read_edgelist(path: Option<&Path>) -> Result<Graph> {
    let reader: Box<dyn BufRead> = match path {
        None => Box::new(BufReader::new(std::io::stdin())),
        Some(p) if p == Path::new("-") => Box::new(BufReader::new(std::io::stdin())),
        Some(p) => Box::new(BufReader::new(File::open(p).map_err(|e| {
            RsomicsError::Io(std::io::Error::new(
                e.kind(),
                format!("{}: {e}", p.display()),
            ))
        })?)),
    };

    let mut labels: Vec<String> = Vec::new();
    let mut index: HashMap<String, u32> = HashMap::new();
    let mut raw_edges: Vec<(u32, u32)> = Vec::new();

    for (lineno, line) in reader.lines().enumerate() {
        let lineno = lineno + 1;
        let line = line.map_err(RsomicsError::Io)?;
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let mut tokens = t.split_ascii_whitespace();
        let u_str = tokens.next().unwrap();
        let v_str = tokens.next().ok_or_else(|| {
            RsomicsError::InvalidInput(format!("line {lineno}: expected two node labels, got one"))
        })?;

        let u = intern(&mut labels, &mut index, u_str);
        let v = intern(&mut labels, &mut index, v_str);
        if u_str == v_str {
            continue;
        }
        raw_edges.push((u, v));
    }

    let n = labels.len();
    let mut adj: Vec<Vec<u32>> = vec![Vec::new(); n];
    for (u, v) in raw_edges {
        adj[u as usize].push(v);
        adj[v as usize].push(u);
    }
    for nbrs in &mut adj {
        nbrs.sort_unstable();
        nbrs.dedup();
    }

    Ok(Graph { labels, adj })
}

fn intern(labels: &mut Vec<String>, index: &mut HashMap<String, u32>, s: &str) -> u32 {
    if let Some(&id) = index.get(s) {
        return id;
    }
    let id = labels.len() as u32;
    labels.push(s.to_owned());
    index.insert(s.to_owned(), id);
    id
}

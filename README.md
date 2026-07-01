# rsomics-graph-shortest-paths

Shortest-path metrics for undirected graphs — a value-exact, faster Rust
reimplementation of networkx's BFS-based path-length functions.

Used in PPI (protein–protein interaction) and co-expression network topology
analysis.

## Usage

```
rsomics-graph-shortest-paths [OPTIONS] --<METRIC> [EDGELIST]
```

Edge list format: one `u v` pair per line, whitespace-separated. Lines
starting with `#` are comments. Blank lines, self-loops, and duplicate
edges are ignored. Only nodes appearing as edge endpoints exist in the graph
(matches `nx.read_edgelist` semantics).

### Metrics

```bash
# Maximum eccentricity (longest shortest path). Errors if disconnected.
rsomics-graph-shortest-paths --diameter graph.el

# Average shortest path length Σ d(u,v) / (n*(n-1)). Errors if disconnected.
rsomics-graph-shortest-paths --average graph.el

# Single-source BFS distances from a node (node TAB dist, sorted by insertion order).
rsomics-graph-shortest-paths --source A graph.el

# Per-node maximum distance (node TAB eccentricity). Errors if disconnected.
rsomics-graph-shortest-paths --eccentricity graph.el

# Minimum eccentricity. Errors if disconnected.
rsomics-graph-shortest-paths --radius graph.el

# Nodes with eccentricity == radius (one label per line).
rsomics-graph-shortest-paths --center graph.el

# Nodes with eccentricity == diameter (one label per line).
rsomics-graph-shortest-paths --periphery graph.el

# Nodes minimising the total distance sum Σ_u d(v,u) (one label per line).
rsomics-graph-shortest-paths --barycenter graph.el

# JSON output (all metrics support --json)
rsomics-graph-shortest-paths --diameter --json graph.el
```

## Accuracy

BFS unweighted distances are integers, so:

- `--diameter`, `--eccentricity` and `--radius` are **integer-exact**.
- `--center`, `--periphery` and `--barycenter` return **exact node sets**
  (center = eccentricity == radius; periphery = eccentricity == diameter;
  barycenter = argmin of the total distance sum Σ_u d(v,u)).
- `--average` computes the exact integer sum Σ d(u,v), then divides once by
  n*(n-1) with a single IEEE-754 double division — **bit-exact** with networkx
  on IEEE-conformant hardware.
- `--source` distances are **integer-exact** per node.

Verified by compat tests against frozen networkx 3.6.1 expectations (including
`to_bits()` float comparisons), plus independent set/integer checks of
`center`/`periphery`/`radius`/`barycenter` against networkx on eight further
connected graphs (0 differ).

## Performance

vs. networkx 3.6.1 (pure-Python BFS), single-threaded, Apple M2:

| Fixture       | Metric   | Ours      | networkx   | Ratio  |
|---------------|----------|-----------|------------|--------|
| gnm100 (n=100, m=300) | diameter | 2.3 ms | 146.9 ms | **62.7×** |
| gnm100 (n=100, m=300) | average  | 2.6 ms | 149.9 ms | **57.0×** |
| gnm500 (n=500, m=3000) | diameter | 7.6 ms | 282.0 ms | **37.0×** |
| gnm500 (n=500, m=3000) | average  | 10.8 ms | 247.4 ms | **22.9×** |

See `PERF_NOTES.md` for full provenance.

## Origin

This crate is an independent Rust reimplementation of networkx shortest-path
metrics based on:

- The networkx 3.6.1 source (BSD 3-Clause):
  `networkx/algorithms/shortest_paths/unweighted.py` and
  `networkx/algorithms/distance_measures.py`
- The standard BFS algorithm for unweighted graphs

The networkx source is MIT/BSD-licensed — reading it for algorithm reference
is permitted. Test golden files were generated from real networkx 3.6.1 output
and are frozen (tests do not call Python at runtime).

License: MIT OR Apache-2.0  
Upstream credit: [networkx](https://networkx.org/) (BSD 3-Clause).

## Install

```bash
cargo install rsomics-graph-shortest-paths
```

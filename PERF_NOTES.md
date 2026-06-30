# Performance Notes

## Machine

- Apple M2 (mini_m2), macOS 15.x, aarch64
- Single-threaded (no parallelism in this crate)

## Upstream reference

- networkx 3.6.1, Python 3.12.13
- Command: `python nx_<metric>.py` (loads graph + computes metric + exits)

## Our build

- rsomics-graph-shortest-paths 0.1.0
- `cargo build --release`
- CARGO_TARGET_DIR=/Volumes/KIOXIA/Developments/cargo-target

## Fixtures

### gnm100_300

`nx.gnm_random_graph(n=100, m=300, seed=42)` — 100 nodes, 300 edges, connected.
Stored at `tests/golden/gnm100_300.el`.

| Metric     | Ours (mean ± σ) | networkx (mean ± σ) | Ratio   |
|------------|-----------------|---------------------|---------|
| diameter   | 2.3 ± 1.3 ms    | 146.9 ± 18.7 ms     | **62.7×** |
| average    | 2.6 ± 0.7 ms    | 149.9 ± 22.0 ms     | **57.0×** |

Note: gnm100 wall time dominated by process startup (~1-2 ms). The graph
compute itself is sub-millisecond.

### gnm500_3000

`nx.gnm_random_graph(n=500, m=3000, seed=456)` — 500 nodes, 3000 edges, connected.
Stored at `/Volumes/KIOXIA/tmp/gnm500_3000.el` (not committed; generate via
`python -c "import networkx as nx; G=nx.gnm_random_graph(500,3000,seed=456); [print(u,v) for u,v in sorted(G.edges())]"`).

| Metric     | Ours (mean ± σ) | networkx (mean ± σ) | Ratio   |
|------------|-----------------|---------------------|---------|
| diameter   | 7.6 ± 0.4 ms    | 282.0 ± 62.4 ms     | **37.0×** |
| average    | 10.8 ± 5.4 ms   | 247.4 ± 17.1 ms     | **22.9×** |

networkx compute-only (no startup): diameter ~144 ms, average ~142 ms.
Our compute-only: diameter ~6 ms, average ~9 ms (subtracting ~1-2 ms startup).

## Why we win

networkx BFS is pure Python with per-node dict allocations and interpreter
overhead at every adjacency lookup. Our BFS is compiled Rust with a flat
`Vec<u32>` adjacency list and a `VecDeque<u32>` frontier — cache-friendly
and zero interpreter overhead.

## Perf gate verdict

>1.0× on all metrics on both fixtures. Release gate: PASS.

//! BFS-based shortest-path metrics for unweighted undirected graphs.
//!
//! Mirrors networkx 3.6.1 `single_source_shortest_path_length` /
//! `all_pairs_shortest_path_length` / `diameter` / `average_shortest_path_length`
//! / `eccentricity` (BSD source read for algorithm reference).
//!
//! All distances are integer-exact; average = integer_sum / (n*(n-1)) with one
//! IEEE-754 division — bit-exact with networkx on IEEE-conformant hardware.

use std::collections::VecDeque;

use crate::io::Graph;

/// BFS distances from `src` to all reachable nodes.
///
/// Returns a Vec indexed by internal node ID; unreachable nodes get `u32::MAX`.
pub fn bfs_distances(g: &Graph, src: u32) -> Vec<u32> {
    let n = g.n();
    let mut dist = vec![u32::MAX; n];
    dist[src as usize] = 0;
    let mut queue = VecDeque::with_capacity(n);
    queue.push_back(src);
    while let Some(u) = queue.pop_front() {
        let d = dist[u as usize];
        for &v in &g.adj[u as usize] {
            if dist[v as usize] == u32::MAX {
                dist[v as usize] = d + 1;
                queue.push_back(v);
            }
        }
    }
    dist
}

/// Check connectivity: returns true iff every node is reachable from node 0.
///
/// An empty graph (n=0) is considered not connected — callers must handle this
/// before calling diameter/average, matching networkx which raises on empty/disconnected.
pub fn is_connected(g: &Graph) -> bool {
    if g.n() == 0 {
        return false;
    }
    let dist = bfs_distances(g, 0);
    dist.iter().all(|&d| d != u32::MAX)
}

/// Single-source BFS distances as a sorted vec of (label, distance) pairs.
///
/// Only reachable nodes are included (unreachable → omitted, matching networkx).
pub fn single_source(g: &Graph, src: u32) -> Vec<(u32, u32)> {
    let dist = bfs_distances(g, src);
    let mut out: Vec<(u32, u32)> = dist
        .iter()
        .enumerate()
        .filter_map(|(i, &d)| {
            if d != u32::MAX {
                Some((i as u32, d))
            } else {
                None
            }
        })
        .collect();
    out.sort_unstable_by_key(|&(node, _)| node);
    out
}

/// Eccentricity of each node: max BFS distance to any other node.
///
/// Requires a connected graph; panics otherwise (callers check connectivity).
pub fn eccentricities(g: &Graph) -> Vec<u32> {
    let n = g.n();
    let mut ecc = vec![0u32; n];
    for src in 0..n as u32 {
        let dist = bfs_distances(g, src);
        ecc[src as usize] = dist.iter().copied().max().unwrap_or(0);
    }
    ecc
}

/// Diameter: maximum eccentricity.
///
/// Errors if graph is disconnected or empty (matching networkx NetworkXError).
pub fn diameter(g: &Graph) -> Result<u32, String> {
    if g.n() == 0 {
        return Err("diameter is undefined for the null graph".into());
    }
    if !is_connected(g) {
        return Err("Found infinite path length because the graph is not connected".into());
    }
    Ok(eccentricities(g).into_iter().max().unwrap_or(0))
}

/// Average shortest path length: Σ d(u,v) / (n*(n-1)).
///
/// Errors if graph is disconnected or empty (matching networkx NetworkXError).
/// Uses one IEEE-754 division on the exact integer sum — bit-exact with networkx.
pub fn average_shortest_path_length(g: &Graph) -> Result<f64, String> {
    let n = g.n();
    if n == 0 {
        return Err("average shortest path length is undefined for the null graph".into());
    }
    if !is_connected(g) {
        return Err("Graph is not connected.".into());
    }
    let mut total: u64 = 0;
    for src in 0..n as u32 {
        let dist = bfs_distances(g, src);
        for &d in &dist {
            if d != u32::MAX {
                total += d as u64;
            }
        }
    }
    // Exact integer sum divided once by n*(n-1), matching networkx.
    Ok(total as f64 / (n as f64 * (n - 1) as f64))
}

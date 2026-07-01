//! Value-exact compat tests against frozen networkx 3.6.1 expectations.
//!
//! Frozen expectations were generated once with:
//!   networkx 3.6.1, python 3.12, mac M2 (2026-07-01)
//! Tests MUST NOT call python at runtime — expectations are hard-coded here.

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

fn bin() -> std::path::PathBuf {
    env!("CARGO_BIN_EXE_rsomics-graph-shortest-paths").into()
}

fn golden(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(name)
}

fn run_ok(flag: &str, file: &str) -> String {
    let out = Command::new(bin())
        .args([flag, golden(file).to_str().unwrap()])
        .output()
        .expect("binary failed to launch");
    assert!(
        out.status.success(),
        "binary exited non-zero for {file} {flag}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

fn run_source(node: &str, file: &str) -> String {
    let out = Command::new(bin())
        .args(["--source", node, golden(file).to_str().unwrap()])
        .output()
        .expect("binary failed to launch");
    assert!(
        out.status.success(),
        "binary exited non-zero for source={node} {file}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

fn run_expect_fail(flag: &str, file: &str) {
    let out = Command::new(bin())
        .args([flag, golden(file).to_str().unwrap()])
        .output()
        .expect("binary failed to launch");
    assert!(
        !out.status.success(),
        "expected non-zero exit for {file} {flag}"
    );
}

fn parse_table(s: &str) -> HashMap<String, u32> {
    s.lines()
        .map(|l| {
            let mut p = l.splitn(2, '\t');
            let node = p.next().unwrap().to_owned();
            let val: u32 = p.next().unwrap().parse().unwrap();
            (node, val)
        })
        .collect()
}

// ── path6 ──────────────────────────────────────────────────────────────────
// nx.path_graph(6): diameter=5, avg=2.3333333333333335 (70/30), ecc={0:5,1:4,2:3,3:3,4:4,5:5}
// single_source from 0: {0:0,1:1,2:2,3:3,4:4,5:5}

#[test]
fn path6_diameter() {
    assert_eq!(run_ok("--diameter", "path6.el"), "5");
}

#[test]
fn path6_average_bit_exact() {
    let s = run_ok("--average", "path6.el");
    let v: f64 = s.parse().unwrap();
    // networkx: 70/30 = 2.3333333333333335 in IEEE-754
    assert_eq!(
        v.to_bits(),
        (2.3333333333333335_f64).to_bits(),
        "average not bit-exact"
    );
}

#[test]
fn path6_eccentricity() {
    let m = parse_table(&run_ok("--eccentricity", "path6.el"));
    assert_eq!(m["0"], 5);
    assert_eq!(m["1"], 4);
    assert_eq!(m["2"], 3);
    assert_eq!(m["3"], 3);
    assert_eq!(m["4"], 4);
    assert_eq!(m["5"], 5);
}

#[test]
fn path6_source_from_0() {
    let m = parse_table(&run_source("0", "path6.el"));
    assert_eq!(m["0"], 0);
    assert_eq!(m["1"], 1);
    assert_eq!(m["2"], 2);
    assert_eq!(m["3"], 3);
    assert_eq!(m["4"], 4);
    assert_eq!(m["5"], 5);
}

// ── star5 ──────────────────────────────────────────────────────────────────
// nx.star_graph(4): diameter=2, avg=1.6 (32/20), ecc={0:1,1:2,2:2,3:2,4:2}
// single_source from 0: {0:0,1:1,2:1,3:1,4:1}

#[test]
fn star5_diameter() {
    assert_eq!(run_ok("--diameter", "star5.el"), "2");
}

#[test]
fn star5_average_bit_exact() {
    let s = run_ok("--average", "star5.el");
    let v: f64 = s.parse().unwrap();
    assert_eq!(v.to_bits(), (1.6_f64).to_bits(), "average not bit-exact");
}

#[test]
fn star5_eccentricity() {
    let m = parse_table(&run_ok("--eccentricity", "star5.el"));
    assert_eq!(m["0"], 1);
    assert_eq!(m["1"], 2);
    assert_eq!(m["2"], 2);
    assert_eq!(m["3"], 2);
    assert_eq!(m["4"], 2);
}

#[test]
fn star5_source_from_0() {
    let m = parse_table(&run_source("0", "star5.el"));
    assert_eq!(m["0"], 0);
    assert_eq!(m["1"], 1);
    assert_eq!(m["2"], 1);
    assert_eq!(m["3"], 1);
    assert_eq!(m["4"], 1);
}

// ── cycle6 ─────────────────────────────────────────────────────────────────
// nx.cycle_graph(6): diameter=3, avg=1.8 (54/30), ecc all=3

#[test]
fn cycle6_diameter() {
    assert_eq!(run_ok("--diameter", "cycle6.el"), "3");
}

#[test]
fn cycle6_average_bit_exact() {
    let s = run_ok("--average", "cycle6.el");
    let v: f64 = s.parse().unwrap();
    assert_eq!(v.to_bits(), (1.8_f64).to_bits(), "average not bit-exact");
}

#[test]
fn cycle6_eccentricity_all_3() {
    let m = parse_table(&run_ok("--eccentricity", "cycle6.el"));
    for node in &["0", "1", "2", "3", "4", "5"] {
        assert_eq!(m[*node], 3, "node {node} eccentricity mismatch");
    }
}

// ── complete4 ──────────────────────────────────────────────────────────────
// nx.complete_graph(4): diameter=1, avg=1.0 (12/12), ecc all=1

#[test]
fn complete4_diameter() {
    assert_eq!(run_ok("--diameter", "complete4.el"), "1");
}

#[test]
fn complete4_average_bit_exact() {
    let s = run_ok("--average", "complete4.el");
    let v: f64 = s.parse().unwrap();
    assert_eq!(v.to_bits(), (1.0_f64).to_bits(), "average not bit-exact");
}

#[test]
fn complete4_eccentricity_all_1() {
    let m = parse_table(&run_ok("--eccentricity", "complete4.el"));
    for node in &["0", "1", "2", "3"] {
        assert_eq!(m[*node], 1, "node {node} eccentricity mismatch");
    }
}

// ── disconnected → errors ───────────────────────────────────────────────────
// two components: 0-1-2 and 10-11
// networkx diameter → NetworkXError; average → NetworkXError

#[test]
fn disconnected_diameter_errors() {
    run_expect_fail("--diameter", "disconnected.el");
}

#[test]
fn disconnected_average_errors() {
    run_expect_fail("--average", "disconnected.el");
}

#[test]
fn disconnected_eccentricity_errors() {
    run_expect_fail("--eccentricity", "disconnected.el");
}

#[test]
fn disconnected_source_reachable_only() {
    // single_source from 0 only sees {0,1,2} — not the 10-11 component
    let m = parse_table(&run_source("0", "disconnected.el"));
    assert_eq!(m["0"], 0);
    assert_eq!(m["1"], 1);
    assert_eq!(m["2"], 2);
    assert!(!m.contains_key("10"), "10 should be unreachable from 0");
    assert!(!m.contains_key("11"), "11 should be unreachable from 0");
}

// ── gnm100_300 (medium connected random graph) ──────────────────────────────
// nx.gnm_random_graph(100, 300, seed=42), networkx 3.6.1
// diameter=5, average=27112/9900=2.7385858585858585
// source from 0: see inline expectations

#[test]
fn gnm100_diameter() {
    assert_eq!(run_ok("--diameter", "gnm100_300.el"), "5");
}

#[test]
fn gnm100_average_bit_exact() {
    let s = run_ok("--average", "gnm100_300.el");
    let v: f64 = s.parse().unwrap();
    // networkx: 27112/9900 = 2.7385858585858585 in IEEE-754
    assert_eq!(
        v.to_bits(),
        (2.7385858585858585_f64).to_bits(),
        "average not bit-exact"
    );
}

#[test]
fn gnm100_eccentricity_selected() {
    let m = parse_table(&run_ok("--eccentricity", "gnm100_300.el"));
    // spot-check: nodes with ecc=5 (networkx confirmed)
    for n in &[
        "2", "10", "23", "32", "36", "37", "41", "49", "50", "53", "61", "62", "63", "75", "79",
        "88", "90", "99",
    ] {
        assert_eq!(m[*n], 5, "node {n} eccentricity should be 5");
    }
    // nodes with ecc=4
    for n in &["0", "1", "3", "4", "5"] {
        assert_eq!(m[*n], 4, "node {n} eccentricity should be 4");
    }
}

#[test]
fn gnm100_source_from_0() {
    let m = parse_table(&run_source("0", "gnm100_300.el"));
    // spot-check selected distances (networkx confirmed)
    assert_eq!(m["0"], 0);
    assert_eq!(m["9"], 1); // 0-9 direct edge
    assert_eq!(m["35"], 1); // 0-35 direct edge
    assert_eq!(m["19"], 4);
    assert_eq!(m["47"], 4);
    assert_eq!(m["61"], 4);
    assert_eq!(m["63"], 4);
    assert_eq!(m["83"], 4);
    assert_eq!(m["88"], 4);
    assert_eq!(m["99"], 4);
    assert_eq!(m.len(), 100, "all 100 nodes reachable from 0");
}

// ── empty graph → errors ────────────────────────────────────────────────────

#[test]
fn empty_graph_diameter_errors() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("empty.el");
    std::fs::write(&p, "# no edges\n").unwrap();
    let out = Command::new(bin())
        .args(["--diameter", p.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!out.status.success(), "must fail on empty graph");
}

#[test]
fn empty_graph_average_errors() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("empty.el");
    std::fs::write(&p, "# no edges\n").unwrap();
    let out = Command::new(bin())
        .args(["--average", p.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!out.status.success(), "must fail on empty graph");
}

#[test]
fn empty_graph_eccentricity_errors() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("empty.el");
    std::fs::write(&p, "# no edges\n").unwrap();
    let out = Command::new(bin())
        .args(["--eccentricity", p.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!out.status.success(), "must fail on empty graph");
}

// ── source → unknown node errors ───────────────────────────────────────────

#[test]
fn source_unknown_node_errors() {
    let out = Command::new(bin())
        .args(["--source", "NOTANODE", golden("path6.el").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!out.status.success(), "must fail for unknown source node");
}

// ── self-loops and duplicates are ignored ──────────────────────────────────

#[test]
fn selfloop_path_unaffected() {
    // path 0-1-2 with a self-loop (1 1) and duplicate (0 1)
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("selfloop.el");
    std::fs::write(&p, "0 1\n1 1\n0 1\n1 2\n").unwrap();
    let out = Command::new(bin())
        .args(["--diameter", p.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8(out.stdout).unwrap().trim(),
        "2",
        "diameter of path 0-1-2 should be 2"
    );
}

// ── helpers for list-output metrics (center / periphery / barycenter) ──────

fn run_list(flag: &str, file: &str) -> Vec<String> {
    let out = Command::new(bin())
        .args([flag, golden(file).to_str().unwrap()])
        .output()
        .expect("binary failed to launch");
    assert!(
        out.status.success(),
        "binary exited non-zero for {file} {flag}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let s = String::from_utf8(out.stdout).unwrap();
    let mut v: Vec<String> = s.lines().map(str::to_owned).collect();
    v.sort();
    v
}

fn run_scalar(flag: &str, file: &str) -> String {
    run_ok(flag, file)
}

// ── radius ─────────────────────────────────────────────────────────────────
// nx: path6→3, star5→1, cycle6→3, complete4→1, gnm100_300→4
// petersen→2, grid2x5→3

#[test]
fn path6_radius() {
    assert_eq!(run_scalar("--radius", "path6.el"), "3");
}

#[test]
fn star5_radius() {
    assert_eq!(run_scalar("--radius", "star5.el"), "1");
}

#[test]
fn cycle6_radius() {
    assert_eq!(run_scalar("--radius", "cycle6.el"), "3");
}

#[test]
fn complete4_radius() {
    assert_eq!(run_scalar("--radius", "complete4.el"), "1");
}

#[test]
fn gnm100_radius() {
    assert_eq!(run_scalar("--radius", "gnm100_300.el"), "4");
}

#[test]
fn petersen_radius() {
    assert_eq!(run_scalar("--radius", "petersen.el"), "2");
}

#[test]
fn grid2x5_radius() {
    assert_eq!(run_scalar("--radius", "grid2x5.el"), "3");
}

#[test]
fn disconnected_radius_errors() {
    run_expect_fail("--radius", "disconnected.el");
}

// ── center ─────────────────────────────────────────────────────────────────
// nx (sorted set): path6→[2,3], star5→[0], cycle6→all6, complete4→all4
// gnm100_300: 82 nodes (all ecc=4); petersen→all10; grid2x5→[2,7]

#[test]
fn path6_center() {
    assert_eq!(run_list("--center", "path6.el"), vec!["2", "3"]);
}

#[test]
fn star5_center() {
    assert_eq!(run_list("--center", "star5.el"), vec!["0"]);
}

#[test]
fn cycle6_center() {
    // All 6 nodes have ecc=3=radius
    let got = run_list("--center", "cycle6.el");
    let expected: Vec<String> = vec!["0", "1", "2", "3", "4", "5"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn complete4_center() {
    let got = run_list("--center", "complete4.el");
    let expected: Vec<String> = vec!["0", "1", "2", "3"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn gnm100_center_count() {
    // 82 nodes with ecc=4 (networkx 3.6.1 confirmed)
    let got = run_list("--center", "gnm100_300.el");
    assert_eq!(got.len(), 82, "gnm100_300 center should have 82 nodes");
    // Periphery nodes (ecc=5) must NOT appear in center
    for n in ["2", "10", "23", "32", "36", "37", "41", "49", "50"] {
        assert!(!got.contains(&n.to_owned()), "periphery node {n} in center");
    }
}

#[test]
fn petersen_center() {
    // All 10 nodes have ecc=2=radius
    let got = run_list("--center", "petersen.el");
    assert_eq!(got.len(), 10);
}

#[test]
fn grid2x5_center() {
    // ecc: 0→5, 1→4, 2→3, 3→4, 4→5, 5→5, 6→4, 7→3, 8→4, 9→5; radius=3
    assert_eq!(run_list("--center", "grid2x5.el"), vec!["2", "7"]);
}

#[test]
fn disconnected_center_errors() {
    run_expect_fail("--center", "disconnected.el");
}

// ── periphery ──────────────────────────────────────────────────────────────
// nx (sorted set): path6→[0,5], star5→[1,2,3,4], cycle6→all6, complete4→all4
// gnm100_300: 18 nodes (ecc=5); petersen→all10; grid2x5→[0,4,5,9]

#[test]
fn path6_periphery() {
    assert_eq!(run_list("--periphery", "path6.el"), vec!["0", "5"]);
}

#[test]
fn star5_periphery() {
    assert_eq!(
        run_list("--periphery", "star5.el"),
        vec!["1", "2", "3", "4"]
    );
}

#[test]
fn cycle6_periphery() {
    let got = run_list("--periphery", "cycle6.el");
    let expected: Vec<String> = vec!["0", "1", "2", "3", "4", "5"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn complete4_periphery() {
    let got = run_list("--periphery", "complete4.el");
    let expected: Vec<String> = vec!["0", "1", "2", "3"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn gnm100_periphery() {
    // 18 nodes with ecc=5 (networkx 3.6.1 confirmed — same as eccentricity ecc=5 spot-check)
    let got = run_list("--periphery", "gnm100_300.el");
    assert_eq!(got.len(), 18, "gnm100_300 periphery should have 18 nodes");
    for n in [
        "2", "10", "23", "32", "36", "37", "41", "49", "50", "53", "61", "62", "63", "75", "79",
        "88", "90", "99",
    ] {
        assert!(
            got.contains(&n.to_owned()),
            "node {n} should be in periphery"
        );
    }
}

#[test]
fn petersen_periphery() {
    // All 10 nodes have ecc=2=diameter
    let got = run_list("--periphery", "petersen.el");
    assert_eq!(got.len(), 10);
}

#[test]
fn grid2x5_periphery() {
    // ecc 5 nodes: 0,4,5,9
    assert_eq!(
        run_list("--periphery", "grid2x5.el"),
        vec!["0", "4", "5", "9"]
    );
}

#[test]
fn disconnected_periphery_errors() {
    run_expect_fail("--periphery", "disconnected.el");
}

// ── barycenter ─────────────────────────────────────────────────────────────
// nx (sorted set): path6→[2,3] (dist_sum=9), star5→[0] (4), cycle6→all6 (9)
// complete4→all4 (3), gnm100_300→[33] (228), petersen→all10 (15), grid2x5→[2,7] (17)

#[test]
fn path6_barycenter() {
    // dist_sum node2=9, node3=9; all others higher
    assert_eq!(run_list("--barycenter", "path6.el"), vec!["2", "3"]);
}

#[test]
fn star5_barycenter() {
    // center node 0 has dist_sum=4; leaves have dist_sum>4
    assert_eq!(run_list("--barycenter", "star5.el"), vec!["0"]);
}

#[test]
fn cycle6_barycenter() {
    // All nodes equidistant (vertex-transitive), min_sum=9
    let got = run_list("--barycenter", "cycle6.el");
    let expected: Vec<String> = vec!["0", "1", "2", "3", "4", "5"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn complete4_barycenter() {
    // All nodes equidistant in K4, min_sum=3
    let got = run_list("--barycenter", "complete4.el");
    let expected: Vec<String> = vec!["0", "1", "2", "3"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn gnm100_barycenter() {
    // Only node 33 minimises dist_sum=228 (networkx 3.6.1 confirmed)
    assert_eq!(run_list("--barycenter", "gnm100_300.el"), vec!["33"]);
}

#[test]
fn petersen_barycenter() {
    // Petersen is vertex-transitive: all 10 nodes share the minimum
    let got = run_list("--barycenter", "petersen.el");
    assert_eq!(got.len(), 10);
}

#[test]
fn grid2x5_barycenter() {
    // Nodes 2 and 7 minimise dist_sum=17
    assert_eq!(run_list("--barycenter", "grid2x5.el"), vec!["2", "7"]);
}

#[test]
fn disconnected_barycenter_errors() {
    run_expect_fail("--barycenter", "disconnected.el");
}

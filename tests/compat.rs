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

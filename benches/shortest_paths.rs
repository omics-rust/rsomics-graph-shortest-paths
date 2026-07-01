use criterion::{Criterion, criterion_group, criterion_main};
use rsomics_graph_shortest_paths::{bfs, io};
use std::path::Path;

fn bench_gnm100(c: &mut Criterion) {
    let golden = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/gnm100_300.el");
    let g = io::read_edgelist(Some(&golden)).unwrap();

    c.bench_function("diameter_gnm100", |b| {
        b.iter(|| bfs::diameter(&g).unwrap());
    });

    c.bench_function("average_gnm100", |b| {
        b.iter(|| bfs::average_shortest_path_length(&g).unwrap());
    });

    c.bench_function("eccentricities_gnm100", |b| {
        b.iter(|| bfs::eccentricities(&g));
    });

    c.bench_function("single_source_gnm100_from_0", |b| {
        b.iter(|| bfs::single_source(&g, 0));
    });

    // New metrics — gnm100
    c.bench_function("radius_gnm100", |b| {
        b.iter(|| bfs::radius(&g).unwrap());
    });

    c.bench_function("center_gnm100", |b| {
        b.iter(|| bfs::center(&g).unwrap());
    });

    c.bench_function("periphery_gnm100", |b| {
        b.iter(|| bfs::periphery(&g).unwrap());
    });

    c.bench_function("barycenter_gnm100", |b| {
        b.iter(|| bfs::barycenter(&g).unwrap());
    });
}

fn bench_gnm300(c: &mut Criterion) {
    let golden = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/gnm300_1500.el");
    let g = io::read_edgelist(Some(&golden)).unwrap();

    c.bench_function("center_gnm300", |b| {
        b.iter(|| bfs::center(&g).unwrap());
    });

    c.bench_function("periphery_gnm300", |b| {
        b.iter(|| bfs::periphery(&g).unwrap());
    });

    c.bench_function("barycenter_gnm300", |b| {
        b.iter(|| bfs::barycenter(&g).unwrap());
    });

    c.bench_function("radius_gnm300", |b| {
        b.iter(|| bfs::radius(&g).unwrap());
    });

    c.bench_function("eccentricities_gnm300", |b| {
        b.iter(|| bfs::eccentricities(&g));
    });
}

criterion_group!(benches, bench_gnm100, bench_gnm300);
criterion_main!(benches);

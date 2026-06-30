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
}

criterion_group!(benches, bench_gnm100);
criterion_main!(benches);

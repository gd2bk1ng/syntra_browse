// ================================================================================================
//   SYNTRA KERNEL — BENCHMARK SUITE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        benches/graph_ops.rs
//   Module:      Syntra Kernel — Graph Operations Benchmarks
//   Description: Benchmarks for evaluating graph traversal performance, adjacency resolution,
//                and relationship scanning.
//
//   Notes:
//     - Run with: `cargo bench`
// ================================================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::HashMap;

// -------------------------------------------------------------------------------------------------
// Mock Graph
// -------------------------------------------------------------------------------------------------

fn build_graph(nodes: usize, edges_per_node: usize) -> HashMap<u64, Vec<u64>> {
    let mut graph = HashMap::new();

    for n in 0..nodes as u64 {
        let edges = (1..=edges_per_node as u64).map(|e| (n + e) % nodes as u64).collect();
        graph.insert(n, edges);
    }

    graph
}

fn traverse(graph: &HashMap<u64, Vec<u64>>, start: u64) -> usize {
    let mut visited = 0usize;

    if let Some(neighbors) = graph.get(&start) {
        for n in neighbors {
            visited += *n as usize;
        }
    }

    visited
}

// -------------------------------------------------------------------------------------------------
// Benchmarks
// -------------------------------------------------------------------------------------------------

fn bench_graph_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_ops");

    for &(nodes, edges) in &[(1_000, 5), (10_000, 10), (50_000, 20)] {
        let graph = build_graph(nodes, edges);

        group.throughput(Throughput::Elements(nodes as u64));

        group.bench_with_input(
            BenchmarkId::new("traverse", format!("{}x{}", nodes, edges)),
            &graph,
            |b, g| b.iter(|| traverse(g, 0)),
        );
    }

    group.finish();
}

// -------------------------------------------------------------------------------------------------
// Entrypoints
// -------------------------------------------------------------------------------------------------

criterion_group!(graph_benches, bench_graph_ops);
criterion_main!(graph_benches);

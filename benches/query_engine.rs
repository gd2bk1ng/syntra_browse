// ================================================================================================
//   SYNTRA KERNEL — BENCHMARK SUITE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        benches/query_engine.rs
//   Module:      Syntra Kernel — Query Engine Benchmarks
//   Description: Benchmarks for evaluating query resolution performance, including filtering,
//                scanning, and indexed lookups.
//
//   Notes:
//     - Run with: `cargo bench`
// ================================================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// -------------------------------------------------------------------------------------------------
// Mock Query Engine
// -------------------------------------------------------------------------------------------------

fn linear_scan(data: &[u64], target: u64) -> usize {
    data.iter().filter(|&&v| v == target).count()
}

fn indexed_lookup(index: &std::collections::HashMap<u64, usize>, target: u64) -> usize {
    *index.get(&target).unwrap_or(&0)
}

// -------------------------------------------------------------------------------------------------
// Benchmarks
// -------------------------------------------------------------------------------------------------

fn bench_query_engine(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_engine");

    for &size in &[10_000usize, 100_000, 1_000_000] {
        let data: Vec<u64> = (0..size as u64).map(|v| v % 100).collect();
        let mut index = std::collections::HashMap::new();

        for v in &data {
            *index.entry(*v).or_insert(0) += 1;
        }

        let target = 42u64;

        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("linear_scan", size), &data, |b, d| {
            b.iter(|| linear_scan(d, target));
        });

        group.bench_with_input(BenchmarkId::new("indexed_lookup", size), &index, |b, idx| {
            b.iter(|| indexed_lookup(idx, target));
        });
    }

    group.finish();
}

// -------------------------------------------------------------------------------------------------
// Entrypoints
// -------------------------------------------------------------------------------------------------

criterion_group!(query_benches, bench_query_engine);
criterion_main!(query_benches);

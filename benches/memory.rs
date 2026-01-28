// ================================================================================================
//   SYNTRA KERNEL — BENCHMARK SUITE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        benches/memory.rs
//   Module:      Syntra Kernel — Memory Benchmarks
//   Description: Benchmarks for evaluating memory access patterns, allocation behavior, and
//                contiguous vs. fragmented data performance.
//
//   Notes:
//     - Run with: `cargo bench`
// ================================================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// -------------------------------------------------------------------------------------------------
// Workloads
// -------------------------------------------------------------------------------------------------

fn contiguous_access(size: usize) {
    let data: Vec<u64> = (0..size as u64).collect();
    let mut sum = 0u64;

    for v in &data {
        sum += *v;
    }

    std::hint::black_box(sum);
}

fn fragmented_access(size: usize) {
    let data: Vec<Box<u64>> = (0..size as u64).map(|v| Box::new(v)).collect();
    let mut sum = 0u64;

    for v in &data {
        sum += **v;
    }

    std::hint::black_box(sum);
}

// -------------------------------------------------------------------------------------------------
// Benchmarks
// -------------------------------------------------------------------------------------------------

fn bench_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_access");

    for &size in &[10_000usize, 100_000, 1_000_000] {
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("contiguous", size), &size, |b, &s| {
            b.iter(|| contiguous_access(s));
        });

        group.bench_with_input(BenchmarkId::new("fragmented", size), &size, |b, &s| {
            b.iter(|| fragmented_access(s));
        });
    }

    group.finish();
}

// -------------------------------------------------------------------------------------------------
// Entrypoints
// -------------------------------------------------------------------------------------------------

criterion_group!(memory_benches, bench_memory);
criterion_main!(memory_benches);

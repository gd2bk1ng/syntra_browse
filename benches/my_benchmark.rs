// ================================================================================================
//   SYNTRA KERNEL — BENCHMARK SUITE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        benches/my_benchmark.rs
//   Module:      Syntra Kernel — Performance Benchmarks
//   Description: Criterion-based microbenchmarks for evaluating isolated computational workloads.
//                This file serves as a template for future benchmarking of world-model updates,
//                agent scheduling, memory access patterns, and cognitive pipeline operations.
//
//   Notes:
//     - Run with: `cargo bench`
//     - All benchmarks follow Syntra Kernel's uniform header and formatting standards.
//     - Recursive Fibonacci is intentionally slow to demonstrate variance.
//     - Iterative Fibonacci provides a stable baseline for comparison.
// ================================================================================================

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};

// -------------------------------------------------------------------------------------------------
// Workloads
// -------------------------------------------------------------------------------------------------

/// Slow recursive Fibonacci — intentionally exponential.
/// Useful for demonstrating Criterion's variance tracking.
fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

/// Fast iterative Fibonacci — stable and predictable.
/// Useful for benchmarking throughput and tight loops.
fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }

    a
}

// -------------------------------------------------------------------------------------------------
// Benchmark Definitions
// -------------------------------------------------------------------------------------------------

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci_comparison");

    // Recursive Fibonacci for small n
    for &n in &[10u64, 20, 25] {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(BenchmarkId::new("recursive", n), &n, |b, &n| {
            b.iter(|| fibonacci_recursive(n));
        });
    }

    // Iterative Fibonacci for larger n
    for &n in &[100u64, 1_000, 10_000] {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(BenchmarkId::new("iterative", n), &n, |b, &n| {
            b.iter(|| fibonacci_iterative(n));
        });
    }

    group.finish();
}

// -------------------------------------------------------------------------------------------------
// Criterion Configuration
// -------------------------------------------------------------------------------------------------

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)
        .warm_up_time(std::time::Duration::from_secs(2))
        .measurement_time(std::time::Duration::from_secs(5))
}

// -------------------------------------------------------------------------------------------------
// Entrypoints
// -------------------------------------------------------------------------------------------------

criterion_group! {
    name = benches;
    config = configure_criterion();
    targets = bench_fibonacci
}

criterion_main!(benches);

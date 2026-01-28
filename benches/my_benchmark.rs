// ================================================================================================
//   SYNTRA KERNEL — BENCHMARK SUITE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        benches/my_benchmark.rs
//   Module:      Syntra Kernel — Performance Benchmarks
//   Description: Criterion-based microbenchmarks for validating performance characteristics of
//                isolated subsystems. These examples serve as a template for future benchmarks
//                involving world-model updates, agent scheduling, and cognitive pipeline tasks.
//
//   Notes:
//     - Run with: `cargo bench`
//     - Criterion provides statistical analysis, warmup cycles, and variance tracking.
//     - These benchmarks are intentionally simple but structured for future expansion.
// ================================================================================================

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};

// -------------------------------------------------------------------------------------------------
// Example Workloads
// -------------------------------------------------------------------------------------------------

/// A deliberately slow recursive Fibonacci for demonstrating variance.
fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

/// A fast iterative Fibonacci for comparison.
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

    // Benchmark recursive Fibonacci for small n
    for &n in &[10u64, 20, 25] {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(BenchmarkId::new("recursive", n), &n, |b, &n| {
            b.iter(|| fibonacci_recursive(n));
        });
    }

    // Benchmark iterative Fibonacci for larger n
    for &n in &[100u64, 1_000, 10_000] {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(BenchmarkId::new("iterative", n), &n, |b, &n| {
            b.iter(|| fibonacci_iterative(n));
        });
    }

    group.finish();
}

// -------------------------------------------------------------------------------------------------
// Criterion Configuration (Optional)
// -------------------------------------------------------------------------------------------------

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(50)          // More samples for stable results
        .warm_up_time(std::time::Duration::from_secs(2))
        .measurement_time(std::time::Duration::from_secs(5))
}

// -------------------------------------------------------------------------------------------------
// Benchmark Entrypoints
// -------------------------------------------------------------------------------------------------

criterion_group! {
    name = benches;
    config = configure_criterion();
    targets = bench_fibonacci
}

criterion_main!(benches);

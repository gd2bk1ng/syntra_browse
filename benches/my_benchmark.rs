// Add Benchmarkingv-vCreate a benches/ directory at the root with benchmark files using Criterion.rs.

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_example(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

criterion_group!(benches, bench_example);
criterion_main!(benches);

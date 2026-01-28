// ================================================================================================
//   SYNTRA KERNEL — BENCHMARK SUITE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        benches/agent_scheduler.rs
//   Module:      Syntra Kernel — Agent Scheduler Benchmarks
//   Description: Benchmarks for evaluating task scheduling, agent dispatch, and queue throughput.
//                These represent core cognitive scheduling operations within the Syntra Kernel.
//
//   Notes:
//     - Run with: `cargo bench`
//     - Simulates realistic agent workloads.
// ================================================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::VecDeque;

// -------------------------------------------------------------------------------------------------
// Mock Scheduler
// -------------------------------------------------------------------------------------------------

fn simulate_agent_tasks(task_count: usize) {
    let mut queue: VecDeque<u64> = (0..task_count as u64).collect();
    let mut processed = 0u64;

    while let Some(task) = queue.pop_front() {
        processed += task % 7;
    }

    std::hint::black_box(processed);
}

// -------------------------------------------------------------------------------------------------
// Benchmarks
// -------------------------------------------------------------------------------------------------

fn bench_scheduler(c: &mut Criterion) {
    let mut group = c.benchmark_group("agent_scheduler");

    for &tasks in &[1_000usize, 10_000, 100_000] {
        group.throughput(Throughput::Elements(tasks as u64));
        group.bench_with_input(BenchmarkId::new("task_dispatch", tasks), &tasks, |b, &t| {
            b.iter(|| simulate_agent_tasks(t));
        });
    }

    group.finish();
}

// -------------------------------------------------------------------------------------------------
// Entrypoints
// -------------------------------------------------------------------------------------------------

criterion_group!(scheduler_benches, bench_scheduler);
criterion_main!(scheduler_benches);

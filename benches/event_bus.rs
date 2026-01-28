// ================================================================================================
//   SYNTRA KERNEL — BENCHMARK SUITE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        benches/event_bus.rs
//   Module:      Syntra Kernel — Event Bus Benchmarks
//   Description: Benchmarks for evaluating message passing throughput, event dispatch, and
//                subscriber fan-out performance.
//
//   Notes:
//     - Run with: `cargo bench`
// ================================================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// -------------------------------------------------------------------------------------------------
// Mock Event Bus
// -------------------------------------------------------------------------------------------------

fn dispatch_events(event_count: usize, subscribers: usize) {
    let mut total = 0usize;

    for event in 0..event_count {
        for sub in 0..subscribers {
            total += (event + sub) % 7;
        }
    }

    std::hint::black_box(total);
}

// -------------------------------------------------------------------------------------------------
// Benchmarks
// -------------------------------------------------------------------------------------------------

fn bench_event_bus(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_bus");

    for &(events, subs) in &[(1_000, 10), (5_000, 50), (10_000, 100)] {
        group.throughput(Throughput::Elements(events as u64));

        group.bench_with_input(
            BenchmarkId::new("dispatch", format!("{}x{}", events, subs)),
            &(events, subs),
            |b, &(e, s)| b.iter(|| dispatch_events(e, s)),
        );
    }

    group.finish();
}

// -------------------------------------------------------------------------------------------------
// Entrypoints
// -------------------------------------------------------------------------------------------------

criterion_group!(event_bus_benches, bench_event_bus);
criterion_main!(event_bus_benches);

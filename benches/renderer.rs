// ================================================================================================
//   SYNTRA KERNEL — BENCHMARK SUITE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        benches/renderer.rs
//   Module:      Syntra Kernel — Renderer Benchmarks
//   Description: Benchmarks for evaluating frame generation, buffer updates, and lightweight
//                rendering simulation.
//
//   Notes:
//     - Run with: `cargo bench`
// ================================================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// -------------------------------------------------------------------------------------------------
// Mock Renderer
// -------------------------------------------------------------------------------------------------

fn render_frame(pixels: usize) {
    let mut buffer = vec![0u8; pixels];

    for px in &mut buffer {
        *px = (*px).wrapping_add(1);
    }

    std::hint::black_box(buffer);
}

// -------------------------------------------------------------------------------------------------
// Benchmarks
// -------------------------------------------------------------------------------------------------

fn bench_renderer(c: &mut Criterion) {
    let mut group = c.benchmark_group("renderer");

    for &px in &[10_000usize, 100_000, 1_000_000] {
        group.throughput(Throughput::Bytes(px as u64));
        group.bench_with_input(BenchmarkId::new("frame", px), &px, |b, &p| {
            b.iter(|| render_frame(p));
        });
    }

    group.finish();
}

// -------------------------------------------------------------------------------------------------
// Entrypoints
// -------------------------------------------------------------------------------------------------

criterion_group!(renderer_benches, bench_renderer);
criterion_main!(renderer_benches);

// ================================================================================================
//   SYNTRA KERNEL — BENCHMARK SUITE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        benches/world_model.rs
//   Module:      Syntra Kernel — World Model Benchmarks
//   Description: Benchmarks for evaluating world-model update performance, including state merging,
//                entity updates, and structural diff application. These represent core cognitive
//                operations within the Syntra Kernel architecture.
//
//   Notes:
//     - Run with: `cargo bench`
//     - These benchmarks simulate realistic world-model workloads.
//     - Designed for future expansion as the world-model subsystem evolves.
// ================================================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::HashMap;

// -------------------------------------------------------------------------------------------------
// Mock World Model Structures
// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
struct Entity {
    id: u64,
    components: HashMap<String, i64>,
}

fn update_entity(entity: &mut Entity) {
    for value in entity.components.values_mut() {
        *value += 1;
    }
}

fn merge_worlds(a: &mut Vec<Entity>, b: &[Entity]) {
    for entity in b {
        a.push(entity.clone());
    }
}

// -------------------------------------------------------------------------------------------------
// Benchmarks
// -------------------------------------------------------------------------------------------------

fn bench_world_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("world_model_updates");

    for &size in &[100usize, 1_000, 10_000] {
        let mut world: Vec<Entity> = (0..size)
            .map(|i| Entity {
                id: i as u64,
                components: HashMap::from([
                    ("health".into(), 100),
                    ("energy".into(), 50),
                    ("xp".into(), 0),
                ]),
            })
            .collect();

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("entity_update", size), &size, |b, _| {
            b.iter(|| {
                for entity in &mut world {
                    update_entity(entity);
                }
            });
        });
    }

    group.finish();
}

fn bench_world_merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("world_model_merge");

    for &size in &[100usize, 1_000, 5_000] {
        let world_a = vec![];
        let world_b: Vec<Entity> = (0..size)
            .map(|i| Entity {
                id: i as u64,
                components: HashMap::from([("value".into(), i as i64)]),
            })
            .collect();

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("merge", size), &size, |b, _| {
            b.iter(|| {
                let mut a = world_a.clone();
                merge_worlds(&mut a, &world_b);
            });
        });
    }

    group.finish();
}

// -------------------------------------------------------------------------------------------------
// Entrypoints
// -------------------------------------------------------------------------------------------------

criterion_group!(world_benches, bench_world_updates, bench_world_merge);
criterion_main!(world_benches);

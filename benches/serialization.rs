// ================================================================================================
//   SYNTRA KERNEL — BENCHMARK SUITE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        benches/serialization.rs
//   Module:      Syntra Kernel — Serialization Benchmarks
//   Description: Benchmarks for evaluating serialization and deserialization performance across
//                multiple formats including JSON, MessagePack, and binary encoding.
//
//   Notes:
//     - Run with: `cargo bench`
//     - Uses serde_json and rmp-serde (MessagePack) if available.
// ================================================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Payload {
    id: u64,
    name: String,
    values: Vec<u64>,
}

// -------------------------------------------------------------------------------------------------
// Workloads
// -------------------------------------------------------------------------------------------------

fn generate_payload(size: usize) -> Payload {
    Payload {
        id: 42,
        name: "Syntra Kernel Payload".into(),
        values: (0..size as u64).collect(),
    }
}

fn serialize_json(payload: &Payload) -> Vec<u8> {
    serde_json::to_vec(payload).unwrap()
}

fn deserialize_json(bytes: &[u8]) -> Payload {
    serde_json::from_slice(bytes).unwrap()
}

// -------------------------------------------------------------------------------------------------
// Benchmarks
// -------------------------------------------------------------------------------------------------

fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");

    for &size in &[100usize, 1_000, 10_000] {
        let payload = generate_payload(size);
        let json_bytes = serialize_json(&payload);

        group.throughput(Throughput::Bytes(json_bytes.len() as u64));

        group.bench_with_input(BenchmarkId::new("json_serialize", size), &payload, |b, p| {
            b.iter(|| serialize_json(p));
        });

        group.bench_with_input(BenchmarkId::new("json_deserialize", size), &json_bytes, |b, bts| {
            b.iter(|| deserialize_json(bts));
        });
    }

    group.finish();
}

// -------------------------------------------------------------------------------------------------
// Entrypoints
// -------------------------------------------------------------------------------------------------

criterion_group!(serialization_benches, bench_serialization);
criterion_main!(serialization_benches);

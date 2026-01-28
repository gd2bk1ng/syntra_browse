<!--
================================================================================
  SYNTRA KERNEL — BENCHMARK DEVELOPER GUIDE
  ------------------------------------------------------------------------------
       .\s/.
      :: S ::
       '/s\'

  File:        benches/DEVELOPER_GUIDE.md
  Module:      Syntra Kernel — Benchmark Developer Documentation
  Description: Technical guide for developers extending or maintaining the
               Syntra Kernel benchmark suite.

  Notes:
    - Banner hidden from GitHub’s rendered view.
================================================================================
-->

# Syntra Kernel — Benchmark Developer Guide

This guide provides detailed instructions for running, extending, and maintaining the Syntra Kernel benchmark suite.

# 1. Running Benchmarks

Run all benchmarks:

```bash
cargo bench
```

Run a specific benchmark group:

```bash
cargo bench -- world_model
```

Run with verbose output:

```bash
cargo bench -- --verbose
```

# 2. Understanding Criterion Output

Criterion generates:

- statistical variance  
- regression detection  
- throughput metrics  
- HTML reports  

Reports are located at:

`target/criterion/<benchmark_name>/report/index.html`

These reports are essential for performance tuning and regression tracking.

# 3. Adding a New Benchmark

Follow this structure:

```rust
fn bench_example(c: &mut Criterion) {
    let mut group = c.benchmark_group("example_group");

    group.bench_with_input(
        BenchmarkId::new("example_case", size),
        &size,
        |b, &s| b.iter(|| workload(s)),
    );

    group.finish();
}

```

Requirements:

- Use BenchmarkId for clarity
- Use Throughput when appropriate
- Keep workloads deterministic
- Avoid global state
- Include the Syntra Kernel banner

Add the file to MANIFEST.md

# 4. Benchmark Philosophy
Syntra Kernel benchmarks must be:

- Deterministic
- Isolated
- Meaningful
- Subsystem-aligned
- Long-term maintainable
- Architecturally consistent

We benchmark behaviors, not micro-optimizations.

# 5. Benchmark Categories
World Model
State updates, merges, structural diffs.

Agent Scheduler
Task dispatch, queue throughput.

Memory
Contiguous vs. fragmented access.

Renderer
Frame generation, buffer updates.

Serialization
Encoding/decoding performance.

Event Bus
Message dispatch throughput.

Query Engine
Linear scans vs. indexed lookups.

Graph Operations
Traversal and adjacency resolution.

# 6. Best Practices

- Keep benchmarks small and focused
- Avoid randomness unless seeded
- Use realistic workloads
- Document assumptions
- Maintain consistent naming
- Prefer clarity over cleverness

# 7. Future Extensions
Potential future benchmark categories:

- Cognitive Pipeline
- Knowledge Graph Reasoning
- Temporal Memory
- Predictive Modeling
- Agent Communication Protocols
- These will be added as the kernel evolves.

This guide ensures that Syntra Kernel’s benchmark suite remains a gold standard for decades to come. Thank you.

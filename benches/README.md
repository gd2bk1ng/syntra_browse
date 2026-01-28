<!--
================================================================================
  SYNTRA KERNEL — BENCHMARK README
  ------------------------------------------------------------------------------
       .\s/.
      :: S ::
       '/s\'

  File:        benches/README.md
  Module:      Syntra Kernel — Benchmark Documentation
  Description: Overview of the Syntra Kernel benchmark suite, including purpose,
               structure, and usage instructions.

  Notes:
    - Banner hidden from GitHub’s rendered view.
    - Only the content below this comment is visible publicly.
================================================================================
-->

# Syntra Kernel — Benchmark Suite

The `/benches/` directory contains all Criterion-based performance benchmarks for the Syntra Kernel. These benchmarks measure the performance characteristics of core subsystems including:

- World Model  
- Agent Scheduler  
- Memory Access  
- Renderer  
- Serialization  
- Event Bus  
- Query Engine  
- Graph Operations  

Each benchmark is isolated, deterministic, and designed for long-term maintainability.

---

## Running Benchmarks

Use Cargo’s built-in benchmarking command:

```bash
cargo bench
```

Criterion will:

- warm up the benchmark  
- run statistical sampling  
- compute variance  
- generate reports in `target/criterion/`  

Open the HTML reports for detailed analysis:

`target/criterion/report/index.html`


---

## Benchmark Structure

Each benchmark file follows a consistent pattern:

1. **Syntra Kernel header banner**  
2. **Workload definitions**  
3. **Benchmark groups**  
4. **Criterion configuration**  
5. **Entrypoints (`criterion_group!`, `criterion_main!`)**

This ensures uniformity across the entire ecosystem.

---

## Extending the Benchmark Suite

To add a new benchmark:

1. Create a new file in `/benches/`  
2. Add the Syntra Kernel banner  
3. Use Criterion’s `BenchmarkId` and `Throughput`  
4. Add the file to `MANIFEST.md`  
5. Ensure the benchmark is meaningful and subsystem-aligned  

For detailed guidance, see `DEVELOPER_GUIDE.md`.

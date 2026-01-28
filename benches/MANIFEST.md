<!--
================================================================================
  SYNTRA KERNEL — BENCHMARK MANIFEST
  ------------------------------------------------------------------------------
       .\s/.
      :: S ::
       '/s\'

  File:        benches/MANIFEST.md
  Module:      Syntra Kernel — Benchmark Manifest
  Description: High-level index of all benchmark suites within the Syntra Kernel
               performance harness. This manifest provides a structured overview
               of each benchmark file, its purpose, and the subsystem it tests.

  Notes:
    - This file is intentionally hidden from GitHub’s rendered view.
    - Developers should consult this manifest before adding new benchmarks.
================================================================================
-->

# Syntra Kernel — Benchmark Manifest

This manifest provides a structured overview of all benchmark suites included in the Syntra Kernel. Each benchmark targets a specific subsystem or architectural concern.

---

## **Benchmark Suites**

### **1. `my_benchmark.rs`**
**Purpose:**  
Introductory Criterion benchmarks demonstrating recursive vs. iterative workloads.

**Subsystem:**  
General computation / template for new benchmarks.

---

### **2. `world_model.rs`**
**Purpose:**  
Benchmarks entity updates, world merging, and structural state operations.

**Subsystem:**  
World Model / Cognitive State Representation.

---

### **3. `agent_scheduler.rs`**
**Purpose:**  
Benchmarks task dispatch, queue throughput, and agent scheduling behavior.

**Subsystem:**  
Agent Scheduler / Cognitive Task Management.

---

### **4. `memory.rs`**
**Purpose:**  
Benchmarks contiguous vs. fragmented memory access patterns.

**Subsystem:**  
Memory Access / Data Layout Performance.

---

### **5. `renderer.rs`**
**Purpose:**  
Benchmarks frame generation, buffer updates, and lightweight rendering simulation.

**Subsystem:**  
Renderer / Visualization Pipeline.

---

### **6. `serialization.rs`**
**Purpose:**  
Benchmarks JSON serialization and deserialization performance.

**Subsystem:**  
Serialization / Data Encoding.

---

### **7. `event_bus.rs`**
**Purpose:**  
Benchmarks message dispatch throughput and subscriber fan-out.

**Subsystem:**  
Event Bus / Messaging Infrastructure.

---

### **8. `query_engine.rs`**
**Purpose:**  
Benchmarks linear scans vs. indexed lookups.

**Subsystem:**  
Query Engine / Data Retrieval.

---

### **9. `graph_ops.rs`**
**Purpose:**  
Benchmarks graph traversal and adjacency resolution.

**Subsystem:**  
Graph Operations / Relationship Modeling.

---

## **Adding New Benchmarks**

When adding a new benchmark:

1. Follow the Syntra Kernel banner format.  
2. Use Criterion’s recommended patterns (`BenchmarkId`, `Throughput`, groups).  
3. Add the new file to this manifest.  
4. Ensure the benchmark is meaningful, measurable, and subsystem‑aligned.

---

This manifest ensures long-term clarity and maintainability for future developers.

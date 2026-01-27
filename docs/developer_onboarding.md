# Developer Onboarding Guide  
*Welcome to the Syntra Kernel architecture*

This guide is designed to help new contributors understand the structure, philosophy, and workflow of Syntra Kernel.  
It complements the main README and the Wiki.

---

# 🧠 1. Understand the Architecture

Before writing code, explore:

- README.md  
- docs/architecture.md  
- docs/cortex_lobes.md  
- docs/world_model_runtime.md  
- docs/memory_architecture.md  
- docs/syntra_language_2.0.md  

Syntra Kernel is a cognitive operating system — understanding the cognitive model is essential.

---

# 🧩 2. Repository Structure

Key directories:

- `src/` — core kernel  
- `src/cortex/` — cognitive lobes  
- `src/runtime/` — multi‑agent runtime  
- `src/memory/` — memory engine  
- `src/world_model/` — world‑model runtime  
- `frontend/` — dashboard UI  
- `trials/` — early cognitive tests  
- `examples/` — minimal runnable examples  
- `docs/` — research corpus  

---

# 🛠 3. Development Environment

### Install dependencies:

- Rust 1.70+  
- Node.js 18+  
- Python 3.x  
- Cargo  
- npm  

### Build the kernel:

```bash
cargo build
```

### Run the kernel:

```bash
cargo run
```

### Run the dashboard:

```bash
cd frontend
npm install
npm start
```

---

# 🧪 4. Running Tests & Examples

### Tests:

```bash
cargo test
```

### Benchmarks:

```bash
cargo bench
```

### Examples:

```bash
cargo run --example renderer_example
```

### Trials:

```bash
cargo run --bin first_intent
```

---

# 🧬 5. Coding Standards

- Follow `rustfmt.toml`  
- Keep modules small and composable  
- Document cognitive behavior clearly  
- Avoid anthropomorphism in comments  
- Write tests for new functionality  
- Use clear, intentional naming  

---

# 🔍 6. Debugging & Introspection

Use:

```bash
cargo run --bin inspect
```

This provides:

- world‑model snapshots  
- memory inspection  
- agent state  
- ThoughtStream queries  

---

# 🤝 7. Contributing Workflow

1. Open an issue describing your idea  
2. Discuss architecture alignment  
3. Submit a PR with tests + docs  
4. Keep changes modular  
5. Follow the cognitive design principles  

---

# 🌟 Welcome to the Architecture

You are now part of a project exploring the frontier of cognitive systems.  
Build boldly — and with clarity.

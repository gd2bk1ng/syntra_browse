<!-- ================================================================================================
     SYNTRA BROWSER — AXIOM ZERO
     ------------------------------------------------------------------------------------------------
     File:        docs/architecture.md
     Module:      System Architecture Overview
     Author:      Alexandr Roussinov (gd2bk1ng)
     Description: High‑level architectural overview of the Syntra Browser. This document describes
                  the core modules, their responsibilities, and how they interact as a cohesive,
                  intent‑driven system.

     Notes:
       This document is intended as the primary reference for new contributors and future system
       architects. Keep it accurate and up to date with major architectural changes.
     ================================================================================================ -->

# Syntra Architecture Overview

Syntra Browser — Axiom Zero is an **intent‑driven cognitive system** wrapped in a graphical shell.
Its architecture is modular, layered, and designed to evolve into a multi‑crate, multi‑process
ecosystem.

---

## Core Modules

### 1. Genesis (`src/genesis.rs`)
**Role:** System bootstrap & awakening sequence.  
**Responsibilities:**
- Create the primary window and pixel surface  
- Run the main event loop  
- Delegate UI rendering to the Cortex navigation lobe  
- Handle shutdown and window events  

Genesis is intentionally thin: it orchestrates, but does not “think”.

---

### 2. AGI Core (`src/agi_core/`)
**Role:** Cognitive primitives and reasoning engines.  
**Key components:**
- `Intent` — semantic representation of user/system intent  
- `Reasoner` — trait for all reasoning engines  
- `NullReasoner` — baseline no‑op reasoner  
- `HeuristicReasoner` — simple refinement engine  

All higher‑level cognition builds on this module.

---

### 3. Conduit (`src/conduit/`)
**Role:** Inter‑module communication backbone.  
**Key components:**
- `ConduitMessage` — typed messages (Log, Intent, Shutdown)  
- `Conduit` — MPSC‑based message channel  

The Conduit is Syntra’s nervous system.

---

### 4. Cortex (`src/cortex/`)
**Role:** Cognitive orchestration layer.  
**Key components:**
- `Cortex<R>` — orchestrator over any `Reasoner`  
- `handle_intent` — processes raw input into refined intent  
- `pump_messages` — consumes conduit messages  
- `nav_lobe` — UI/navigation lobe used by Genesis  

The Cortex is the conductor of Syntra’s cognitive orchestra.

---

### 5. Renderer (`src/renderer/`)
**Role:** Visual output pipeline abstraction.  
**Key components:**
- `Renderer` trait — unified rendering interface  
- `NullRenderer` — placeholder implementation  

Future versions will integrate GPU pipelines and advanced UI composition.

---

### 6. Utilities (`src/utilities/`)
**Role:** Shared helpers and cross‑cutting tools.  
**Key components:**
- `timestamp` — RFC3339 UTC timestamp  
- `log_info`, `log_warn`, `log_error` — structured logging  

Utilities should remain lightweight and focused.

---

## High‑Level Flow

1. `main.rs` calls `syntra_browse::genesis::main()`.
2. Genesis:
   - Creates window + pixel surface  
   - Enters event loop  
   - Delegates drawing to `cortex::nav_lobe::draw_ui()`  
3. Future:
   - Input events → Cortex → AGI Core → Renderer  
   - Conduit routes messages between subsystems  

---

## Design Principles

- **Intent‑first**: everything revolves around user/system intent.  
- **Modular**: each module can evolve into its own crate.  
- **Observable**: logs and messages are first‑class citizens.  
- **Future‑proof**: architecture designed for decades of evolution.  

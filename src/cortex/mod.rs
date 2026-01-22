/* ================================================================================================
   SYNTRA BROWSER — AXIOM FOUR
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/cortex/mod.rs
   Module:      Cortex (Cognitive Orchestration Layer)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: The Cortex is Syntra’s high‑level cognitive conductor. It integrates the AGI Core
                (intent semantics), the Conduit (message bus), and all cognitive lobes introduced
                across Axiom Zero → Axiom Four. The Cortex receives raw user input, refines it
                through the Reasoner, routes it to the appropriate lobe, stores memory, manages
                a self‑modification sandbox, and returns structured responses.

   Overview:
     • Cortex<R>        — Generic orchestrator over any Reasoner implementation.
     • handle_intent    — Legacy Axiom Zero/One intent dispatch.
     • process          — Axiom Three/Four cognitive pipeline (classification → routing → memory).
     • pump_messages    — Polls the Conduit for logs, intents, and shutdown signals.

   Integrated Lobes:
     • Request Lobe       — High-level request classification (Axiom Two).
     • Memory Lobe        — Stores intents + responses (Axiom Two).
     • Knowledge Lobe     — Semantic memory from browsing (Axiom Three).
     • Execution Lobe     — Multi-step workflows (Axiom Three).
     • Perception Lobe    — Content parsing and summarization (Axiom Three).
     • Action Lobe        — External actions (fetch, run commands).
     • Reflection Lobe    — Self-analysis and introspection.
     • Plan Lobe          — Multi-step planning.
     • Evolution Lobe     — Architectural improvement narratives.
     • Sandbox Lobe       — In‑memory self‑modification workspace (Axiom Four).
     • Meta‑Evolution     — Higher‑order evolution proposals (Axiom Four).
     • Maintenance Lobe   — System health & recovery knowledge (Axiom Four).
     • nav_lobe           — UI/navigation lobe for future browser surfaces.

   Notes:
     - The Cortex is intentionally modular and ASCII-safe.
     - It is the central nervous system of Syntra’s cognition.
     - Axiom Four introduces a safe self‑modification sandbox; no direct writes to disk occur here.
   ================================================================================================ */

#![allow(dead_code)]

/* ------------------------------------------------------------------------------------------------
   MODULE DECLARATIONS
   ------------------------------------------------------------------------------------------------ */

pub mod request_lobe;
pub mod memory_lobe;
pub mod plan_lobe;
pub mod reflection_lobe;
pub mod evolution_lobe;
pub mod perception_lobe;
pub mod action_lobe;
pub mod knowledge_lobe;
pub mod execution_lobe;
pub mod sandbox_lobe;
pub mod meta_evolution_lobe;
pub mod maintenance_lobe;

pub use request_lobe::{Request, RequestKind, RequestLobe};
pub use memory_lobe::{MemoryLobe, MemoryEntry};
pub use plan_lobe::PlanLobe;
pub use reflection_lobe::ReflectionLobe;
pub use evolution_lobe::EvolutionLobe;
pub use perception_lobe::{PerceptionLobe, Perception};
pub use action_lobe::{ActionLobe, ActionResult};
pub use knowledge_lobe::{KnowledgeLobe, KnowledgeEntry};
pub use execution_lobe::{ExecutionLobe, Task, TaskStep};
pub use sandbox_lobe::{SandboxSession, SandboxFile, SandboxPatch};
pub use meta_evolution_lobe::{MetaEvolutionLobe, EvolutionProposal, FileChange};
pub use maintenance_lobe::MaintenanceLobe;

pub mod nav_lobe;

/* ------------------------------------------------------------------------------------------------
   IMPORTS
   ------------------------------------------------------------------------------------------------ */

use crate::agi_core::{Intent, Reasoner, NullReasoner, IntentPlan};
use crate::conduit::{Conduit, ConduitMessage};
use crate::utilities::log_info;

/* ------------------------------------------------------------------------------------------------
   CORTEX STRUCTURE
   ------------------------------------------------------------------------------------------------ */

/// The Cortex orchestrates high‑level system behavior, routing intents and coordinating
/// subsystems such as the renderer, AGI core, and all cognitive lobes.
pub struct Cortex<R: Reasoner = NullReasoner> {
    pub reasoner: R,
    pub conduit: Conduit,
    pub memory: MemoryLobe,
    pub knowledge: KnowledgeLobe,
    pub sandbox: SandboxSession,
}

impl<R: Reasoner> Cortex<R> {
    /// Construct a new Cortex with a Reasoner and Conduit.
    pub fn new(reasoner: R, conduit: Conduit) -> Self {
        Self {
            reasoner,
            conduit,
            memory: MemoryLobe::new(),
            knowledge: KnowledgeLobe::new(),
            sandbox: SandboxSession::new(),
        }
    }

    /* --------------------------------------------------------------------------------------------
       AXIOM ZERO / ONE — LEGACY INTENT HANDLER
       -------------------------------------------------------------------------------------------- */

    /// Legacy: refine intent and push to conduit.
    /// Still used by older components and for backward compatibility.
    pub fn handle_intent(&self, raw: &str) {
        log_info(&format!("Cortex received raw intent: {}", raw));

        let intent = Intent {
            label: raw.to_string(),
            confidence: 0.9,
        };

        let refined = self.reasoner.process(intent);

        log_info(&format!(
            "Cortex refined intent: {} (class: {}, confidence: ~{:.2})",
            refined.intent, refined.class, 0.9
        ));

        self.conduit.send(ConduitMessage::Intent(refined.intent));
    }

    /* --------------------------------------------------------------------------------------------
       AXIOM THREE / FOUR — FULL COGNITIVE PIPELINE
       -------------------------------------------------------------------------------------------- */

    /// Axiom Three/Four: full cognitive processing pipeline.
    /// Returns a human-readable response string.
    pub fn process(&mut self, raw: &str) -> String {
        log_info(&format!("Cortex::process received: {}", raw));

        let intent = Intent {
            label: raw.to_string(),
            confidence: 0.9,
        };

        let plan: IntentPlan = self.reasoner.process(intent);

        log_info(&format!(
            "Cortex classified intent as '{}' with plan: {}",
            plan.class, plan.plan
        ));

        // Store the intent in memory.
        self.memory.store_intent(&plan.intent, &plan.class);

        /* ----------------------------------------------------------------------------------------
           ROUTE TO LOBES
           ---------------------------------------------------------------------------------------- */

        let response = match plan.class.as_str() {
            /* ------------------------------------------------------------------------------------
               MAINTENANCE — System Health & Recovery (Axiom Four)
               ------------------------------------------------------------------------------------ */
            "maintenance" => {
                let arg = plan
                    .intent
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(" ");
                match arg.as_str() {
                    "rust"       => MaintenanceLobe::rust_toolchain(),
                    "cargo"      => MaintenanceLobe::cargo_cache(),
                    "git"        => MaintenanceLobe::git_recovery(),
                    "shell"      => MaintenanceLobe::shell_integrity(),
                    "reinstall"  => MaintenanceLobe::full_reinstall(),
                    "full"       => MaintenanceLobe::all(),
                    _            => MaintenanceLobe::overview(),
                }
            }

            /* ------------------------------------------------------------------------------------
               BROWSE — Fetch + Perceive + Store
               ------------------------------------------------------------------------------------ */
            "browse" => {
                // Expect: "browse <url>"
                let mut parts = plan.intent.split_whitespace();
                let _ = parts.next(); // consume "browse"
                if let Some(first) = parts.next() {
                    // Allow URLs with spaces (e.g., quoted or multi-part)
                    let url = std::iter::once(first)
                        .chain(parts)
                        .collect::<Vec<_>>()
                        .join(" ");
                    let result = ActionLobe::fetch_url(&url);
                    if result.success {
                        let perception = PerceptionLobe::perceive(&result.output);
                        self.knowledge.store(&url, perception.clone());
                        format!(
                            "Fetched and perceived '{}'.\nTitle: {:?}\nSummary:\n{}",
                            url,
                            perception.title,
                            perception.summary
                        )
                    } else {
                        format!("Failed to fetch URL: {}", url)
                    }
                } else {
                    "Usage: browse <url>".to_string()
                }
            }

            /* ------------------------------------------------------------------------------------
               KNOWLEDGE — Semantic Memory Search
               ------------------------------------------------------------------------------------ */
            "knowledge" => {
                // Expect: "knowledge <query>" or "search <query>"
                let query = plan
                    .intent
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(" ");
                if query.is_empty() {
                    "Usage: knowledge <query>".to_string()
                } else {
                    let results = self.knowledge.search(&query);
                    if results.is_empty() {
                        format!("No knowledge entries found matching '{}'.", query)
                    } else {
                        let mut out = format!("Knowledge matches for '{}':\n", query);
                        for (i, entry) in results.iter().enumerate() {
                            out.push_str(&format!(
                                "  [{}] source: {}\n      title: {:?}\n",
                                i + 1,
                                entry.source,
                                entry.perception.title
                            ));
                        }
                        out
                    }
                }
            }

            /* ------------------------------------------------------------------------------------
               TASK — Multi-Step Execution
               ------------------------------------------------------------------------------------ */
            "task" => {
                // Expect: "task <name>"
                let name = plan
                    .intent
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(" ");
                if name.is_empty() {
                    "Usage: task <name>".to_string()
                } else {
                    let task = Task {
                        name: format!("Demo task for '{}'", name),
                        steps: vec![TaskStep::FetchAndPerceive { url: name }],
                    };
                    let log = ExecutionLobe::run(&task, &mut self.knowledge);
                    format!("Task executed.\n{}", log)
                }
            }

            /* ------------------------------------------------------------------------------------
               PERCEPTION — Parse & Summarize Text
               ------------------------------------------------------------------------------------ */
            "perception" => {
                // Expect: "perceive <text>"
                let text = plan
                    .intent
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(" ");
                if text.is_empty() {
                    "Usage: perceive <text>".to_string()
                } else {
                    let perception = PerceptionLobe::perceive(&text);
                    format!(
                        "Perception:\nTitle: {:?}\nHeadings: {:?}\nSummary:\n{}",
                        perception.title, perception.headings, perception.summary
                    )
                }
            }

            /* ------------------------------------------------------------------------------------
               ACTION — System Commands
               ------------------------------------------------------------------------------------ */
            "action" => {
                // Expect: "act <cmd> [args...]"
                let mut parts = plan.intent.split_whitespace().skip(1);
                if let Some(cmd) = parts.next() {
                    let args: Vec<&str> = parts.collect();
                    let result = ActionLobe::run_command(cmd, &args);
                    if result.success {
                        format!("Command '{}' succeeded.\nOutput:\n{}", cmd, result.output)
                    } else {
                        format!("Command '{}' failed or produced no output.", cmd)
                    }
                } else {
                    "Usage: act <cmd> [args...]".to_string()
                }
            }

            /* ------------------------------------------------------------------------------------
               EVOLUTION — Architectural Proposals (Axiom Four)
               ------------------------------------------------------------------------------------ */
            "evolution" => {
                let proposal = MetaEvolutionLobe::generate_proposal(&plan.intent);
                MetaEvolutionLobe::describe_proposal(&proposal)
            }

            /* ------------------------------------------------------------------------------------
               SANDBOX — Self‑Modification Workspace Introspection
               ------------------------------------------------------------------------------------ */
            "sandbox" => {
                // Expect: "sandbox diff" or "sandbox snapshot"
                let cmd = plan
                    .intent
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join(" ");
                match cmd.as_str() {
                    "diff" => self.sandbox.diff(),
                    "snapshot" => {
                        let files = self.sandbox.snapshot();
                        if files.is_empty() {
                            "Sandbox is empty.".to_string()
                        } else {
                            let mut out = String::new();
                            for file in files {
                                out.push_str(&format!("File: {}\n", file.path));
                                out.push_str("  --- content (truncated) ---\n");
                                let preview: String = file
                                    .content
                                    .lines()
                                    .take(8)
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                out.push_str(&preview);
                                out.push_str("\n\n");
                            }
                            out
                        }
                    }
                    _ => {
                        "Sandbox commands:\n  sandbox diff\n  sandbox snapshot".to_string()
                    }
                }
            }

            /* ------------------------------------------------------------------------------------
               PLANNING — Multi-Step Plans
               ------------------------------------------------------------------------------------ */
            "planning" => {
                PlanLobe::generate_plan(&plan.intent)
            }

            /* ------------------------------------------------------------------------------------
               SELF-REFLECTION — Introspection
               ------------------------------------------------------------------------------------ */
            "self_reflection" => {
                ReflectionLobe::self_reflect()
            }

            /* ------------------------------------------------------------------------------------
               FREEFORM — General Reflection
               ------------------------------------------------------------------------------------ */
            "freeform" => {
                ReflectionLobe::reflect(&plan.intent)
            }

            /* ------------------------------------------------------------------------------------
               FALLBACK
               ------------------------------------------------------------------------------------ */
            other => {
                format!(
                    "I classified this as '{}' but have no handler yet.\nPlan: {}",
                    other, plan.plan
                )
            }
        };

        // Store response in memory.
        self.memory.store_response(&response);

        response
    }

    /* --------------------------------------------------------------------------------------------
       CONDUIT MESSAGE PUMP
       -------------------------------------------------------------------------------------------- */

    /// Polls the conduit for messages and logs them.
    pub fn pump_messages(&self) {
        while let Some(msg) = self.conduit.try_recv() {
            match msg {
                ConduitMessage::Log(text) => log_info(&format!("[Conduit] {}", text)),
                ConduitMessage::Intent(label) => {
                    log_info(&format!("[Conduit] Intent dispatched: {}", label))
                }
                ConduitMessage::Shutdown => {
                    log_info("[Conduit] Shutdown signal received.");
                    break;
                }
            }
        }
    }
}

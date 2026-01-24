/* ================================================================================================
   SYNTRA BROWSER — AXIOM FIVE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/agi_core/intent.rs
   Module:      AGI Core — Intent Semantics Engine
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Unified semantic intent engine for Syntra’s emerging AGI Core. This module
                classifies freeform text into structured intent classes, generates high-level
                plans, supports probabilistic reasoning hints, and provides safe ASCII output for
                the intent bridge. It merges Axiom Zero’s deterministic classifier with Axiom
                Three/Four/Five’s expanded semantic domains.

   Overview:
     • Intent           — Raw user intent with confidence metadata.
     • IntentPlan       — Classified intent with semantic class + high-level plan.
     • Reasoner         — Dual-interface trait for pluggable reasoning engines.
     • NullReasoner     — Deterministic baseline classifier.
     • ProbReasoner     — Probabilistic, multi-step planning wrapper.
     • IntentLog        — Ring buffer for introspection.
     • debug_plan       — One-line summary for overlays/terminals.
   ================================================================================================ */

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/* ------------------------------------------------------------------------------------------------
   DATA STRUCTURES
   ------------------------------------------------------------------------------------------------ */

/// Raw user intent with confidence metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub label: String,
    pub confidence: f32,
}

impl Intent {
    pub fn new(label: impl Into<String>, confidence: f32) -> Self {
        Self {
            label: label.into(),
            confidence,
        }
    }
}

/// Classified intent with semantic domain + high-level plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentPlan {
    pub intent: String,
    pub class: String,
    pub plan: String,
    /// Optional multi-step plan, when available.
    pub steps: Vec<String>,
    /// Optional probability estimate (0.0–1.0) for classification confidence.
    pub probability: f32,
}

/* ------------------------------------------------------------------------------------------------
   TRAIT: Reasoner (Dual Interface)
   ------------------------------------------------------------------------------------------------ */

pub trait Reasoner {
    /// Full semantic reasoning (Axiom Five).
    fn process(&self, intent: Intent) -> IntentPlan;

    /// Simple text reasoning (Axiom Three compatibility).
    fn reason_text(&self, intent: &Intent) -> String {
        self.process(intent.clone()).plan
    }
}

/* ------------------------------------------------------------------------------------------------
   NULL REASONER (Deterministic Baseline)
   ------------------------------------------------------------------------------------------------ */

pub struct NullReasoner;

impl NullReasoner {
    pub fn new() -> Self {
        Self
    }
}

impl Reasoner for NullReasoner {
    fn process(&self, intent: Intent) -> IntentPlan {
        let class = classify_domain(&intent.label);
        let plan = plan_for_domain(&class, &intent.label);
        let steps = multi_step_plan(&class, &intent.label);
        let probability = baseline_probability(&class, intent.confidence);

        IntentPlan {
            intent: intent.label,
            class,
            plan,
            steps,
            probability,
        }
    }
}

/* ------------------------------------------------------------------------------------------------
   PROBABILISTIC / MULTI-STEP REASONER
   ------------------------------------------------------------------------------------------------ */

/// A thin wrapper that can, in the future, incorporate real probabilistic models.
/// For now, it decorates NullReasoner with richer multi-step planning semantics.
pub struct ProbReasoner {
    inner: NullReasoner,
}

impl ProbReasoner {
    pub fn new() -> Self {
        Self {
            inner: NullReasoner::new(),
        }
    }
}

impl Reasoner for ProbReasoner {
    fn process(&self, intent: Intent) -> IntentPlan {
        // For now, delegate to NullReasoner; future axioms can adjust probability/steps.
        self.inner.process(intent)
    }
}

/* ------------------------------------------------------------------------------------------------
   SEMANTIC CLASSIFICATION (Axiom Five)
   ------------------------------------------------------------------------------------------------ */

pub fn classify_domain(intent: &str) -> String {
    let lower = intent.to_lowercase();

    if lower.starts_with("evaluate ") || lower.starts_with("evaluation ") {
        return "evaluation".to_string();
    }
    if lower.starts_with("maintenance ") {
        return "maintenance".to_string();
    }
    if lower.starts_with("browse ") {
        return "browse".to_string();
    }
    if lower.starts_with("knowledge ") || lower.starts_with("search ") {
        return "knowledge".to_string();
    }
    if lower.starts_with("task ") {
        return "task".to_string();
    }
    if lower.starts_with("perceive ") {
        return "perception".to_string();
    }
    if lower.starts_with("act ") {
        return "action".to_string();
    }

    if lower.contains("evolve") || lower.contains("improve") {
        return "evolution".to_string();
    }
    if lower.contains("plan") || lower.contains("design") || lower.contains("blueprint") {
        return "planning".to_string();
    }
    if lower.contains("self") || lower.contains("syntra") || lower.contains("ecosystem") {
        return "self_reflection".to_string();
    }

    if lower.contains("diagnose") || lower.contains("status") || lower.contains("health") {
        return "diagnostic".to_string();
    }

    if lower.contains("portal") || lower.contains("open") || lower.contains("launch") {
        return "navigation".to_string();
    }

    if lower.contains("build") || lower.contains("create") || lower.contains("generate") {
        return "construction".to_string();
    }

    "freeform".to_string()
}

/* ------------------------------------------------------------------------------------------------
   HIGH-LEVEL PLANNING (Axiom Five)
   ------------------------------------------------------------------------------------------------ */

pub fn plan_for_domain(classification: &str, intent: &str) -> String {
    match classification {
        "evaluation" => {
            "Compare two versions of a module or text, highlight strengths, weaknesses, risks, \
             and provide a qualitative verdict."
                .to_string()
        }
        "maintenance" => {
            "Provide system maintenance guidance: Rust toolchain, Cargo cache, Git recovery, \
             terminal integrity, or full reinstall procedures."
                .to_string()
        }
        "browse" => {
            "Fetch the given URL, perceive its content, store it in the knowledge lobe, \
             and summarize the observed structure."
                .to_string()
        }
        "knowledge" => {
            "Search the knowledge lobe for entries matching the query and return structured results."
                .to_string()
        }
        "task" => {
            "Execute a multi-step workflow via the execution lobe, combining perception, \
             action, and knowledge."
                .to_string()
        }
        "perception" => {
            "Run the perception lobe on the provided text and extract title, headings, links, \
             and a lightweight summary."
                .to_string()
        }
        "action" => {
            "Execute a simple system command via the action lobe and return its output."
                .to_string()
        }
        "evolution" => {
            "Use the evolution and meta-evolution lobes to propose architectural improvements, \
             new lobes, and future axioms."
                .to_string()
        }
        "planning" => {
            "Generate a multi-step plan outlining modules, integration points, tests, \
             and documentation updates."
                .to_string()
        }
        "self_reflection" => {
            "Run self-analysis routines: inspect repository structure, check diagnostics, \
             and describe current capabilities."
                .to_string()
        }
        "diagnostic" => {
            "Probe repository structure, check branches, scan for missing lobes, and report \
             system health without modifying code."
                .to_string()
        }
        "navigation" => {
            "Map the requested concept or location to a future browsing or scene context \
             within Syntra's renderer."
                .to_string()
        }
        "construction" => {
            "Propose a safe scaffold for new modules, including Rust skeletons, tests, and \
             documentation, without writing files yet."
                .to_string()
        }
        _ => {
            format!(
                "Capture this freeform intent for higher-order AGI interpretation. \
                 Intent text: '{}'.",
                intent
            )
        }
    }
}

/* ------------------------------------------------------------------------------------------------
   MULTI-STEP PLANNING (Structured)
   ------------------------------------------------------------------------------------------------ */

pub fn multi_step_plan(classification: &str, intent: &str) -> Vec<String> {
    match classification {
        "evaluation" => vec![
            "Identify the two artifacts or versions to compare.".into(),
            "Extract key structural and behavioral differences.".into(),
            "Assess risks, strengths, and weaknesses.".into(),
            "Summarize a qualitative verdict with rationale.".into(),
        ],
        "planning" => vec![
            "Clarify the target outcome and constraints.".into(),
            "List required modules and integration points.".into(),
            "Define test coverage and validation strategy.".into(),
            "Outline documentation and onboarding updates.".into(),
        ],
        "evolution" => vec![
            "Scan current architecture and lobes.".into(),
            "Identify bottlenecks and missing capabilities.".into(),
            "Propose new lobes or refactors.".into(),
            "Prioritize changes by impact and risk.".into(),
        ],
        "self_reflection" => vec![
            "Inspect current repository structure.".into(),
            "Check diagnostics and recent changes.".into(),
            "Summarize current capabilities and gaps.".into(),
            "Propose next steps for growth.".into(),
        ],
        _ => vec![format!("Handle freeform intent: '{}'.", intent)],
    }
}

/* ------------------------------------------------------------------------------------------------
   PROBABILITY ESTIMATION (Simple Heuristic)
   ------------------------------------------------------------------------------------------------ */

fn baseline_probability(classification: &str, confidence: f32) -> f32 {
    let base = match classification {
        "evaluation" | "planning" | "evolution" | "self_reflection" => 0.85,
        "maintenance" | "diagnostic" | "navigation" | "construction" => 0.8,
        "browse" | "knowledge" | "task" | "perception" | "action" => 0.9,
        _ => 0.6,
    };
    (base * confidence).clamp(0.0, 1.0)
}

/* ------------------------------------------------------------------------------------------------
   DEBUG / INTROSPECTION HELPERS
   ------------------------------------------------------------------------------------------------ */

pub fn debug_plan(plan: &IntentPlan) -> String {
    format!(
        "[class={}] p={:.2} intent=\"{}\" plan=\"{}\"",
        plan.class,
        plan.probability,
        escape_json(&plan.intent),
        escape_json(&plan.plan)
    )
}

/* ------------------------------------------------------------------------------------------------
   INTENT LOG (Ring Buffer)
   ------------------------------------------------------------------------------------------------ */

#[derive(Debug)]
pub struct IntentLog {
    entries: Vec<IntentPlan>,
    capacity: usize,
}

impl IntentLog {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, plan: IntentPlan) {
        if self.entries.len() == self.capacity {
            self.entries.remove(0);
        }
        self.entries.push(plan);
    }

    pub fn entries(&self) -> &[IntentPlan] {
        &self.entries
    }

    pub fn latest(&self) -> Option<&IntentPlan> {
        self.entries.last()
    }
}

/* ------------------------------------------------------------------------------------------------
   JSON ESCAPING (ASCII-Safe)
   ------------------------------------------------------------------------------------------------ */

pub fn escape_json(input: &str) -> String {
    let mut out = String::new();
    for c in input.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (INTENT SEMANTICS ENGINE, ADVANCED)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/intent.rs
//   Module:      Intent Semantics Engine (Advanced)
//   Description: Advanced semantic intent engine for the Syntra Kernel. Provides contextual,
//                feature-aware, hierarchical intent classification, graph-based plans, and
//                feedback-aware confidence modeling, while remaining deterministic and explainable.
//
//   Overview:
//     • Intent / IntentPlan        — Core intent structures.
//     • Context                    — Lightweight contextual state for reasoning.
//     • IntentFeatures             — Deterministic semantic feature extraction.
//     • IntentEmbedding            — Symbolic embedding for similarity and clustering.
//     • Reasoner                   — Trait for pluggable reasoning engines.
//     • NullReasoner               — Deterministic, feature-aware baseline.
//     • ProbReasoner               — Wrapper for future probabilistic models.
//     • classify_domain            — Hierarchical, feature-aware classification.
//     • plan_for_domain            — High-level narrative planning.
//     • multi_step_plan            — Structured, graph-backed multi-step planning.
//     • IntentLog                  — Ring buffer for introspection.
//     • debug_plan                 — One-line summary for overlays/terminals.
// ================================================================================================

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// ================================================================================================
// Core Data Structures
// ================================================================================================

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

/// Lightweight contextual state for reasoning.
/// This is intentionally minimal and ASCII-safe.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Context {
    pub recent_intents: Vec<String>,
    pub active_task: Option<String>,
    pub system_load: f32,
}

/// Symbolic feature representation of an intent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntentFeatures {
    pub verbs: Vec<String>,
    pub nouns: Vec<String>,
    pub has_url: bool,
    pub is_question: bool,
    pub sentiment: f32,
    pub complexity: f32,
}

/// Symbolic embedding for similarity and clustering.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IntentEmbedding {
    pub domain_vector: [f32; 8],
    pub sentiment: f32,
    pub complexity: f32,
}

/// A node in a graph-based plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanNode {
    pub id: usize,
    pub description: String,
    pub next: Vec<usize>,
}

/// Classified intent with semantic domain + high-level plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentPlan {
    pub intent: String,
    pub class: String,
    pub plan: String,
    /// Linearized multi-step plan (for compatibility).
    pub steps: Vec<String>,
    /// Probability estimate (0.0–1.0) for classification confidence.
    pub probability: f32,
    /// Extracted features used during reasoning.
    pub features: IntentFeatures,
    /// Symbolic embedding for similarity and clustering.
    pub embedding: IntentEmbedding,
    /// Optional graph-based plan representation.
    pub graph: Vec<PlanNode>,
    /// Optional alternative classifications for disambiguation.
    pub alternatives: Vec<(String, f32)>,
}

// ================================================================================================
// Reasoner Trait (Dual Interface + Context)
// ================================================================================================

pub trait Reasoner {
    /// Full semantic reasoning with optional context.
    fn process(&self, intent: Intent, ctx: Option<&Context>) -> IntentPlan;

    /// Simple text reasoning (Axiom Three compatibility).
    fn reason_text(&self, intent: &Intent) -> String {
        self.process(intent.clone(), None).plan
    }
}

// ================================================================================================
// Feature Extraction
// ================================================================================================

pub fn extract_features(text: &str) -> IntentFeatures {
    let lower = text.to_lowercase();
    let tokens: Vec<&str> = lower.split_whitespace().collect();

    let mut verbs = Vec::new();
    let mut nouns = Vec::new();
    let mut has_url = false;
    let mut is_question = lower.trim_end().ends_with('?');

    for t in &tokens {
        if t.starts_with("http://") || t.starts_with("https://") {
            has_url = true;
        }
        if ["open", "browse", "search", "plan", "build", "create", "diagnose", "fix", "run"]
            .contains(t)
        {
            verbs.push((*t).to_string());
        } else if ["syntra", "kernel", "renderer", "cortex", "runtime", "task", "module", "file"]
            .contains(t)
        {
            nouns.push((*t).to_string());
        }
    }

    // Very simple sentiment and complexity heuristics.
    let sentiment = if lower.contains("love")
        || lower.contains("great")
        || lower.contains("nice")
        || lower.contains("good")
    {
        0.7
    } else if lower.contains("hate")
        || lower.contains("bad")
        || lower.contains("terrible")
        || lower.contains("off")
    {
        -0.5
    } else {
        0.0
    };

    let complexity = (tokens.len() as f32 / 20.0).min(1.5);

    IntentFeatures {
        verbs,
        nouns,
        has_url,
        is_question,
        sentiment,
        complexity,
    }
}

// ================================================================================================
// Embedding Construction
// ================================================================================================

pub fn build_embedding(class: &str, features: &IntentFeatures) -> IntentEmbedding {
    let mut v = [0.0_f32; 8];

    // Domain encoding (one-hot-ish).
    let domain_index = match class {
        "evaluation" => 0,
        "maintenance" => 1,
        "browse" | "knowledge" => 2,
        "task" | "planning" => 3,
        "perception" => 4,
        "action" => 5,
        "evolution" | "self_reflection" => 6,
        _ => 7,
    };
    v[domain_index] = 1.0;

    // Complexity and question-ness as small tweaks.
    if features.is_question {
        v[domain_index] += 0.1;
    }

    IntentEmbedding {
        domain_vector: v,
        sentiment: features.sentiment,
        complexity: features.complexity,
    }
}

// ================================================================================================
// NullReasoner (Deterministic, Feature-Aware Baseline)
// ================================================================================================

pub struct NullReasoner;

impl NullReasoner {
    pub fn new() -> Self {
        Self
    }
}

impl Reasoner for NullReasoner {
    fn process(&self, intent: Intent, ctx: Option<&Context>) -> IntentPlan {
        let features = extract_features(&intent.label);
        let primary_class = classify_domain(&intent.label, &features, ctx);
        let alternatives = disambiguate_domains(&intent.label, &features, ctx, &primary_class);

        let plan = plan_for_domain(&primary_class, &intent.label);
        let graph = graph_plan_for_domain(&primary_class, &intent.label);
        let steps = linearize_plan_graph(&graph, &intent.label);

        let probability = advanced_probability(
            &primary_class,
            &alternatives,
            &features,
            ctx,
            intent.confidence,
        );

        let embedding = build_embedding(&primary_class, &features);

        IntentPlan {
            intent: intent.label,
            class: primary_class,
            plan,
            steps,
            probability,
            features,
            embedding,
            graph,
            alternatives,
        }
    }
}

// ================================================================================================
// ProbReasoner (Wrapper for Future Probabilistic Models)
// ================================================================================================

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
    fn process(&self, intent: Intent, ctx: Option<&Context>) -> IntentPlan {
        // For now, delegate to NullReasoner; future axioms can adjust probability/steps.
        self.inner.process(intent, ctx)
    }
}

// ================================================================================================
// Hierarchical, Feature-Aware Classification
// ================================================================================================

pub fn classify_domain(
    intent: &str,
    features: &IntentFeatures,
    ctx: Option<&Context>,
) -> String {
    let lower = intent.to_lowercase();

    // Strong lexical prefixes.
    if lower.starts_with("evaluate ") || lower.starts_with("evaluation ") {
        return "evaluation".to_string();
    }
    if lower.starts_with("maintenance ") {
        return "maintenance".to_string();
    }
    if lower.starts_with("browse ") || (features.has_url && features.verbs.contains(&"browse".into()))
    {
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
    if lower.starts_with("act ") || lower.starts_with("run ") {
        return "action".to_string();
    }

    // Contextual hints.
    if let Some(ctx) = ctx {
        if ctx.active_task.is_some() && lower.contains("continue") {
            return "task".to_string();
        }
        if ctx
            .recent_intents
            .iter()
            .any(|i| i.to_lowercase().contains("diagnose"))
            && lower.contains("fix")
        {
            return "maintenance".to_string();
        }
    }

    // Semantic cues.
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

// Generate alternative candidate domains for disambiguation.
pub fn disambiguate_domains(
    intent: &str,
    features: &IntentFeatures,
    ctx: Option<&Context>,
    primary: &str,
) -> Vec<(String, f32)> {
    let mut candidates = Vec::new();
    let lower = intent.to_lowercase();

    let mut push_candidate = |class: &str, score: f32| {
        if class != primary {
            candidates.push((class.to_string(), score));
        }
    };

    if features.has_url {
        push_candidate("browse", 0.7);
        push_candidate("knowledge", 0.6);
    }
    if features.is_question {
        push_candidate("knowledge", 0.65);
        push_candidate("diagnostic", 0.55);
    }
    if lower.contains("fix") || lower.contains("repair") {
        push_candidate("maintenance", 0.7);
    }
    if lower.contains("why") || lower.contains("explain") {
        push_candidate("self_reflection", 0.6);
    }
    if let Some(ctx) = ctx {
        if ctx.active_task.is_some() {
            push_candidate("task", 0.6);
        }
    }

    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    candidates
}

// ================================================================================================
// High-Level Planning (Narrative)
// ================================================================================================

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

// ================================================================================================
// Graph-Based Multi-Step Planning
// ================================================================================================

pub fn graph_plan_for_domain(classification: &str, intent: &str) -> Vec<PlanNode> {
    match classification {
        "evaluation" => vec![
            PlanNode {
                id: 0,
                description: "Identify the two artifacts or versions to compare.".into(),
                next: vec![1],
            },
            PlanNode {
                id: 1,
                description: "Extract key structural and behavioral differences.".into(),
                next: vec![2],
            },
            PlanNode {
                id: 2,
                description: "Assess risks, strengths, and weaknesses.".into(),
                next: vec![3],
            },
            PlanNode {
                id: 3,
                description: "Summarize a qualitative verdict with rationale.".into(),
                next: vec![],
            },
        ],
        "planning" => vec![
            PlanNode {
                id: 0,
                description: "Clarify the target outcome and constraints.".into(),
                next: vec![1],
            },
            PlanNode {
                id: 1,
                description: "List required modules and integration points.".into(),
                next: vec![2],
            },
            PlanNode {
                id: 2,
                description: "Define test coverage and validation strategy.".into(),
                next: vec![3],
            },
            PlanNode {
                id: 3,
                description: "Outline documentation and onboarding updates.".into(),
                next: vec![],
            },
        ],
        "evolution" => vec![
            PlanNode {
                id: 0,
                description: "Scan current architecture and lobes.".into(),
                next: vec![1],
            },
            PlanNode {
                id: 1,
                description: "Identify bottlenecks and missing capabilities.".into(),
                next: vec![2],
            },
            PlanNode {
                id: 2,
                description: "Propose new lobes or refactors.".into(),
                next: vec![3],
            },
            PlanNode {
                id: 3,
                description: "Prioritize changes by impact and risk.".into(),
                next: vec![],
            },
        ],
        "self_reflection" => vec![
            PlanNode {
                id: 0,
                description: "Inspect current repository structure.".into(),
                next: vec![1],
            },
            PlanNode {
                id: 1,
                description: "Check diagnostics and recent changes.".into(),
                next: vec![2],
            },
            PlanNode {
                id: 2,
                description: "Summarize current capabilities and gaps.".into(),
                next: vec![3],
            },
            PlanNode {
                id: 3,
                description: "Propose next steps for growth.".into(),
                next: vec![],
            },
        ],
        _ => vec![PlanNode {
            id: 0,
            description: format!("Handle freeform intent: '{}'.", intent),
            next: vec![],
        }],
    }
}

/// Linearize a graph-based plan into a simple step list for compatibility.
pub fn linearize_plan_graph(graph: &[PlanNode], _intent: &str) -> Vec<String> {
    if graph.is_empty() {
        return Vec::new();
    }

    let mut steps = Vec::new();
    let mut current_id = 0usize;
    let mut visited = std::collections::HashSet::new();

    loop {
        if visited.contains(&current_id) {
            break;
        }
        visited.insert(current_id);

        if let Some(node) = graph.iter().find(|n| n.id == current_id) {
            steps.push(node.description.clone());
            if let Some(&next_id) = node.next.first() {
                current_id = next_id;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    steps
}

// ================================================================================================
// Probability Estimation (Advanced Heuristic)
// ================================================================================================

fn advanced_probability(
    classification: &str,
    alternatives: &[(String, f32)],
    features: &IntentFeatures,
    ctx: Option<&Context>,
    base_confidence: f32,
) -> f32 {
    let base = match classification {
        "evaluation" | "planning" | "evolution" | "self_reflection" => 0.85,
        "maintenance" | "diagnostic" | "navigation" | "construction" => 0.8,
        "browse" | "knowledge" | "task" | "perception" | "action" => 0.9,
        _ => 0.6,
    };

    let alt_penalty = if alternatives.is_empty() { 0.0 } else { 0.1 };

    let sentiment_boost = if features.sentiment > 0.3 {
        0.05
    } else if features.sentiment < -0.3 {
        -0.05
    } else {
        0.0
    };

    let ctx_boost = if let Some(ctx) = ctx {
        if ctx.active_task.is_some() && classification == "task" {
            0.05
        } else {
            0.0
        }
    } else {
        0.0
    };

    let mut p = base * base_confidence;
    p += sentiment_boost + ctx_boost;
    p -= alt_penalty;

    p.clamp(0.0, 1.0)
}

// ================================================================================================
// Debug / Introspection Helpers
// ================================================================================================

pub fn debug_plan(plan: &IntentPlan) -> String {
    let alt_summary = if plan.alternatives.is_empty() {
        "alts=[]".to_string()
    } else {
        let parts: Vec<String> = plan
            .alternatives
            .iter()
            .map(|(c, p)| format!("{}:{:.2}", c, p))
            .collect();
        format!("alts=[{}]", parts.join(","))
    };

    format!(
        "[class={}] p={:.2} {} intent=\"{}\" plan=\"{}\"",
        plan.class,
        plan.probability,
        alt_summary,
        escape_json(&plan.intent),
        escape_json(&plan.plan)
    )
}

// ================================================================================================
// Intent Log (Ring Buffer)
// ================================================================================================

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

// ================================================================================================
// JSON Escaping (ASCII-Safe)
// ================================================================================================

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

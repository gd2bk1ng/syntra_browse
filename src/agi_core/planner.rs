// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (INTENT PLANNER)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/planner.rs
//   Module:      Intent Planner (Narrative + Multi-Step + Graph-Based)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       High-level narrative planning and structured multi-step plan generation for the
//       Intent Semantics Engine. Produces both linear step lists and graph-based plans,
//       used by Cortex, Terminal, and ThoughtStream.
//
//   Architectural Role:
//       • Axiom Three — Cognitive Awareness (structured reasoning).
//       • Axiom Four  — Perception / Cognition / Action routing.
//       • Axiom Six   — Evolution (self-modification planning).
//
//   Notes:
//       - Deterministic, ASCII-safe, explainable.
//       - No side effects, no I/O.
// ================================================================================================

#![allow(dead_code)]

use crate::agi_core::intent::PlanNode;

// ================================================================================================
// Narrative Planning
// ================================================================================================

/// Generate a high-level narrative plan for a given domain.
///
/// This is what gets surfaced as `plan` in the IntentPlan.
pub fn narrative_plan(intent_text: &str, class: &str) -> String {
    match class {
        "browse" => format!(
            "Fetch the requested URL or resource, perceive its contents, store relevant knowledge, \
             and summarize back to the creator. Intent: '{}'.",
            intent_text
        ),

        "knowledge" => format!(
            "Search the knowledge lobe for relevant entries, optionally augment with perception, \
             and return a concise, structured answer. Intent: '{}'.",
            intent_text
        ),

        "task" => format!(
            "Decompose the requested task into ordered steps, execute them via the action lobe, \
             and report progress and results. Intent: '{}'.",
            intent_text
        ),

        "evolve" => format!(
            "Interpret the request as a meta-evolution proposal, analyze the current ecosystem, \
             and generate a safe, staged self-modification plan. Intent: '{}'.",
            intent_text
        ),

        "sandbox" => format!(
            "Inspect the self-modification sandbox, show staged patches or snapshots, and \
             prepare them for review or application. Intent: '{}'.",
            intent_text
        ),

        "introspection" | "self" => format!(
            "Retrieve recent ThoughtStream entries, diagnostics, or ecosystem state to provide \
             a self-reflective view of the kernel. Intent: '{}'.",
            intent_text
        ),

        "ecosystem" => format!(
            "Scan the filesystem ecosystem, compute lobe completeness, and present a structural \
             diagnostic of the Syntra Kernel. Intent: '{}'.",
            intent_text
        ),

        "safety" => format!(
            "Query the active safety policy, list protected lobes, and summarize current \
             governance constraints. Intent: '{}'.",
            intent_text
        ),

        _ => format!(
            "Interpret the input as a freeform cognitive request, reason about it using the \
             cortex and knowledge lobes, and respond in a helpful, structured way. Intent: '{}'.",
            intent_text
        ),
    }
}

// ================================================================================================
// Multi-Step Planning
// ================================================================================================

/// Build a linear multi-step plan and a graph representation.
///
/// The steps are intentionally high-level and generic; they can be refined
/// by downstream lobes (e.g., execution, perception).
pub fn multi_step_plan(class: &str, intent_text: &str) -> (Vec<String>, Vec<PlanNode>) {
    let mut steps = Vec::new();

    match class {
        "browse" => {
            steps.push("Parse URL or target resource from intent.".into());
            steps.push("Fetch content via perception lobe.".into());
            steps.push("Extract salient information into knowledge lobe.".into());
            steps.push("Summarize and present results to the creator.".into());
        }

        "knowledge" => {
            steps.push("Parse query and key entities from intent.".into());
            steps.push("Search knowledge lobe for relevant entries.".into());
            steps.push("Optionally augment with fresh perception if needed.".into());
            steps.push("Synthesize a concise, structured answer.".into());
        }

        "task" => {
            steps.push("Parse task description and constraints.".into());
            steps.push("Decompose into ordered execution steps.".into());
            steps.push("Execute steps via action lobe with safety checks.".into());
            steps.push("Aggregate results and report back.".into());
        }

        "evolve" => {
            steps.push("Interpret evolution request and scope.".into());
            steps.push("Scan ecosystem and current capabilities.".into());
            steps.push("Generate candidate self-modification proposals.".into());
            steps.push("Stage proposals in sandbox for review.".into());
        }

        "sandbox" => {
            steps.push("Inspect current sandbox state.".into());
            steps.push("List staged patches and affected lobes.".into());
            steps.push("Summarize potential impact and safety considerations.".into());
        }

        "introspection" | "self" => {
            steps.push("Gather recent ThoughtStream entries.".into());
            steps.push("Collect diagnostics and ecosystem state.".into());
            steps.push("Summarize internal state in a human-readable form.".into());
        }

        "ecosystem" => {
            steps.push("Scan filesystem for expected lobes.".into());
            steps.push("Compute completeness and health scores.".into());
            steps.push("Generate upgrade recommendations.".into());
        }

        "safety" => {
            steps.push("Load active safety policy.".into());
            steps.push("Enumerate protected lobes and constraints.".into());
            steps.push("Summarize current governance posture.".into());
        }

        _ => {
            steps.push("Interpret freeform intent and extract key goals.".into());
            steps.push("Select appropriate lobes (knowledge, perception, action).".into());
            steps.push("Reason about the request and synthesize a response.".into());
        }
    }

    // Build a simple linear graph: 0 -> 1 -> 2 -> ...
    let mut graph = Vec::new();
    for (i, desc) in steps.iter().enumerate() {
        let next = if i + 1 < steps.len() {
            vec![i + 1]
        } else {
            Vec::new()
        };
        graph.push(PlanNode {
            id: i,
            description: desc.clone(),
            next,
        });
    }

    // Embed the original intent text into the first step.
    if let Some(first) = graph.first_mut() {
        first.description = format!("Intent: '{}'. {}", intent_text, first.description);
    }

    (steps, graph)
}

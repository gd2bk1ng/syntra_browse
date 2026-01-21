// ================================================================================================
// SYNTRA BROWSER - AXIOM ZERO
// ------------------------------------------------------------------------------------------------
// SIGIL:
//       .\s/.
//      :: S ::
//       '/s\'
//
// File:        src/agi_core/intent.rs
// Module:      Syntra AGI Core - Intent Semantics
// Author:      Alexandr Roussinov (gd2bk1ng)
// Description: Core intent classification and planning primitives for Syntra's emerging AGI core.
//              Provides ASCII-safe, dependency-free logic for interpreting freeform text into
//              structured intent classes and high-level plans.
//
// Notes:
//   - Designed to be stable and reusable across binaries (main runtime, intent bridge, tools).
//   - No external crates; safe to embed in constrained environments.
//   - This module is the canonical source of truth for intent semantics in Axiom Zero.
// ================================================================================================

pub fn classify_intent(intent: &str) -> String {
    let lower = intent.to_lowercase();

    if lower.contains("diagnose") || lower.contains("status") || lower.contains("health") {
        "diagnostic".to_string()
    } else if lower.contains("plan") || lower.contains("design") || lower.contains("blueprint") {
        "planning".to_string()
    } else if lower.contains("portal") || lower.contains("open") || lower.contains("launch") {
        "navigation".to_string()
    } else if lower.contains("build") || lower.contains("create") || lower.contains("generate") {
        "construction".to_string()
    } else {
        "freeform".to_string()
    }
}

pub fn plan_for_intent(classification: &str, intent: &str) -> String {
    match classification {
        "diagnostic" => {
            "Probe repository structure, check branches, scan for missing lobes, and report health without modifying code.".to_string()
        }
        "planning" => {
            "Outline modules, integration points, tests, and documentation updates for the requested capability.".to_string()
        }
        "navigation" => {
            "Map the requested concept or location to a future browsing or scene context within Syntra's renderer.".to_string()
        }
        "construction" => {
            "Propose a safe scaffold for new modules, including Rust skeletons, tests, and docs, without writing files yet.".to_string()
        }
        _ => {
            format!(
                "Capture this freeform intent for higher-order AGI interpretation. Intent text: '{}'.",
                intent
            )
        }
    }
}

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

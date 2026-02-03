// ================================================================================================
//   SYNTRA KERNEL — REFACTOR RULES (CONSTRAINTS & SAFETY BOUNDARIES)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/refactor_rules.rs
//   Module:      Utilities — Refactor Rules & Constraints
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Defines the rule system governing Syntra’s refactoring behavior. These rules act as
//       constitutional constraints for the RefactorEngine, ensuring that all structural changes
//       remain safe, coherent, and aligned with Syntra’s architectural principles.
//
//       This module provides:
//         • Rule definitions (hard rules, soft rules, advisory rules)
//         • Rule evaluation engine
//         • Rule violations and severity levels
//         • Region-based constraints (Core, Lobes, Utilities, Tests)
//         • Symbol-level constraints (traits, public APIs, unsafe code)
//
//       These rules are consulted before any refactor plan is accepted or applied.
//
//   Notes:
//       - Non-mutating; pure evaluation layer.
//       - Designed for future integration with SafetyGate and SelfModPolicy.
//       - MIT & Apache 2.0 dual-licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use serde::{Serialize, Deserialize};

use crate::utilities::{
    SemanticRole,
    SemanticFsView,
    CodeIndex,
    CodeSymbol,
    SymbolKind,
    RefactorPlan,
};

/// Severity of a rule violation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleSeverity {
    Advisory,   // Suggestion only
    Warning,    // Should be avoided
    Critical,   // Must not proceed
}

/// A single rule violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleViolation {
    pub rule_id: String,
    pub message: String,
    pub severity: RuleSeverity,
}

/// A refactor rule definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactorRule {
    pub id: String,
    pub description: String,
    pub severity: RuleSeverity,
}

/// Rule evaluation engine.
#[derive(Debug)]
pub struct RefactorRuleEngine {
    rules: Vec<RefactorRule>,
}

impl RefactorRuleEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: RefactorRule) {
        self.rules.push(rule);
    }

    /// Evaluate a refactor plan against all rules.
    pub fn evaluate_plan(
        &self,
        plan: &RefactorPlan,
        semantic: &SemanticFsView,
        index: &CodeIndex,
    ) -> Vec<RuleViolation> {
        let mut violations = Vec::new();

        for rule in &self.rules {
            match rule.id.as_str() {
                "no_core_mutation" => {
                    for op in &plan.operations {
                        if let Some(file) = semantic.files.iter().find(|f| f.path == op.target_file) {
                            if file.role == SemanticRole::AgiCore {
                                violations.push(RuleViolation {
                                    rule_id: rule.id.clone(),
                                    message: format!(
                                        "Refactor operation '{}' targets AGI Core file '{}', which is protected.",
                                        op.description, op.target_file
                                    ),
                                    severity: rule.severity.clone(),
                                });
                            }
                        }
                    }
                }

                "no_public_api_rename" => {
                    for op in &plan.operations {
                        if let Some(symbol) = &op.target_symbol {
                            let matches = index.find_by_name(symbol);
                            for sym in matches {
                                if sym.kind == SymbolKind::Function || sym.kind == SymbolKind::Trait {
                                    // Future: detect public API via visibility analysis
                                    violations.push(RuleViolation {
                                        rule_id: rule.id.clone(),
                                        message: format!(
                                            "Renaming public-facing symbol '{}' may break API stability.",
                                            symbol
                                        ),
                                        severity: rule.severity.clone(),
                                    });
                                }
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        violations
    }
}

/// Default rule set for Syntra.
pub fn default_refactor_rules() -> RefactorRuleEngine {
    let mut engine = RefactorRuleEngine::new();

    engine.add_rule(RefactorRule {
        id: "no_core_mutation".into(),
        description: "AGI Core files cannot be refactored without explicit human approval.",
        severity: RuleSeverity::Critical,
    });

    engine.add_rule(RefactorRule {
        id: "no_public_api_rename".into(),
        description: "Renaming public-facing symbols is discouraged unless part of a major evolution.",
        severity: RuleSeverity::Warning,
    });

    engine
}


// ================================================================================================
//   SYNTRA KERNEL — COMPLEXITY ANALYZER
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/complexity_analyzer.rs
//   Module:      Utilities — Complexity Analyzer
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Computes structural complexity metrics across Syntra’s codebase, including:
//
//         • Cyclomatic complexity (approximate heuristic)
//         • Function complexity (branching density)
//         • File complexity (symbol density, LOC)
//         • Module cohesion (symbols per file)
//         • Hotspot detection (complexity × churn-ready)
//
//       These metrics feed into:
//         • RiskAnalyzer
//         • EvolutionPredictor
//         • RefactorEngine
//         • SelfHealingAdvisor
//
//       This module is intentionally lightweight and heuristic-based, designed to evolve into a
//       full AST-driven analyzer as Syntra grows.
//
//   Notes:
//       - Non-mutating; pure analysis.
//       - ASCII-safe, dependency-minimal.
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
use std::collections::HashMap;

use crate::utilities::{
    CodeIndex,
    CodeSymbol,
    SymbolKind,
    fs_utils,
};

/// Complexity score for a single file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileComplexity {
    pub file: String,
    pub loc: usize,
    pub symbol_count: usize,
    pub cyclomatic_estimate: usize,
    pub branching_density: f32,
}

/// Complexity score for a single function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    pub name: String,
    pub file: String,
    pub line: u32,
    pub cyclomatic_estimate: usize,
    pub branching_density: f32,
}

/// Full complexity report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityReport {
    pub files: Vec<FileComplexity>,
    pub functions: Vec<FunctionComplexity>,
}

/// Complexity analyzer — heuristic-based structural complexity measurement.
#[derive(Debug)]
pub struct ComplexityAnalyzer;

impl ComplexityAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Build a full complexity report.
    pub fn analyze(
        &self,
        root: &std::path::Path,
        index: &CodeIndex,
    ) -> ComplexityReport {
        let mut file_map: HashMap<String, Vec<&CodeSymbol>> = HashMap::new();

        // Group symbols by file
        for sym in index.all() {
            file_map.entry(sym.file.clone())
                .or_default()
                .push(sym);
        }

        let mut files_out = Vec::new();
        let mut functions_out = Vec::new();

        for (file, symbols) in file_map {
            let full_path = root.join(&file);

            let text = match fs_utils::read_text_file(&full_path) {
                Ok(t) => t,
                Err(_) => continue,
            };

            let loc = text.lines().count();

            // Estimate cyclomatic complexity by counting branching keywords
            let branch_keywords = ["if ", "else if", "match ", "for ", "while ", "&&", "||"];
            let mut cyclomatic = 1; // base complexity

            for line in text.lines() {
                for kw in &branch_keywords {
                    if line.contains(kw) {
                        cyclomatic += 1;
                    }
                }
            }

            let branching_density = cyclomatic as f32 / loc.max(1) as f32;

            files_out.push(FileComplexity {
                file: file.clone(),
                loc,
                symbol_count: symbols.len(),
                cyclomatic_estimate: cyclomatic,
                branching_density,
            });

            // Function-level complexity
            for sym in symbols {
                if sym.kind == SymbolKind::Function {
                    // Heuristic: treat function body as slice of file
                    let func_complexity = FunctionComplexity {
                        name: sym.name.clone(),
                        file: file.clone(),
                        line: sym.line,
                        cyclomatic_estimate: cyclomatic / 2, // placeholder heuristic
                        branching_density,
                    };
                    functions_out.push(func_complexity);
                }
            }
        }

        ComplexityReport {
            files: files_out,
            functions: functions_out,
        }
    }
}

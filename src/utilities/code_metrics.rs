// ================================================================================================
//   SYNTRA KERNEL — CODE METRICS ENGINE
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/code_metrics.rs
//   Module:      Utilities — Code Metrics Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Computes quantitative metrics across Syntra’s codebase, including:
//
//         • Lines of code (LOC)
//         • Symbol density
//         • Dependency fan-in / fan-out
//         • File churn potential (heuristic)
//         • Hotspot detection
//
//       These metrics feed into:
//         • ComplexityAnalyzer
//         • RiskAnalyzer
//         • EvolutionPredictor
//         • ChangeImpactAnalyzer
//
//   Notes:
//       - Lightweight and heuristic-based.
//       - Designed for future integration with Git history for real churn metrics.
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
    BaselineSnapshot,
    CodeIndex,
    DependencyGraph,
    fs_utils,
};

/// Metrics for a single file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    pub file: String,
    pub loc: usize,
    pub symbol_count: usize,
    pub fan_in: usize,
    pub fan_out: usize,
    pub hotspot_score: f32,
}

/// Full metrics report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsReport {
    pub files: Vec<FileMetrics>,
}

/// Code metrics engine.
#[derive(Debug)]
pub struct CodeMetricsEngine;

impl CodeMetricsEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(
        &self,
        root: &std::path::Path,
        snapshot: &BaselineSnapshot,
        index: &CodeIndex,
        deps: &DependencyGraph,
    ) -> MetricsReport {
        let mut out = Vec::new();

        for (path, entry) in &snapshot.files {
            if !path.ends_with(".rs") {
                continue;
            }

            let full = root.join(path);
            let text = match fs_utils::read_text_file(&full) {
                Ok(t) => t,
                Err(_) => continue,
            };

            let loc = text.lines().count();

            let symbol_count = index
                .all()
                .iter()
                .filter(|s| s.file == *path)
                .count();

            let fan_in = deps
                .edges
                .iter()
                .filter(|e| e.to == *path)
                .count();

            let fan_out = deps
                .edges
                .iter()
                .filter(|e| e.from == *path)
                .count();

            let hotspot_score = (symbol_count as f32 * 0.4)
                + (fan_in as f32 * 0.3)
                + (fan_out as f32 * 0.3);

            out.push(FileMetrics {
                file: path.clone(),
                loc,
                symbol_count,
                fan_in,
                fan_out,
                hotspot_score,
            });
        }

        MetricsReport { files: out }
    }
}


// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (ECOSYSTEM MODEL + DIAGNOSTIC SCANNER, ADVANCED)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/ecosystem.rs
//   Module:      Ecosystem Model + Diagnostic Scanner (Advanced)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Structural and qualitative model of the Syntra Kernel ecosystem. Scans the repository,
//       computes lobe completeness, and produces upgrade recommendations and an overall health
//       score. This is the AGI Core’s structural introspection layer.
//
//   Architectural Role:
//       • Axiom Two  — Identity & Continuity (structural self-knowledge).
//       • Axiom Three — Behavioral Awareness (knowing where cognition lives).
//       • Axiom Four  — Cognitive / Perception / Action support (ecosystem introspection).
//       • Axiom Six   — Evolution (guides self-modification proposals).
//
//   Notes:
//       - This scanner is intentionally heuristic and filesystem-based.
//       - It does not execute code; it only inspects structure and size.
//       - Future versions may integrate AST analysis, coverage metrics, and semantic graphs.
// ================================================================================================

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

use crate::agi_core::telemetry::{TelemetryBus, TelemetryEvent, TelemetryLevel};

/// High-level category of a lobe.
#[cfg_attr(feature = "agi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum LobeKind {
    Core,
    Runtime,
    Interface,
    Compiler,
    Data,
    Docs,
    Experimental,
}

/// Importance of a lobe for overall system health.
#[cfg_attr(feature = "agi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum Criticality {
    Essential,
    Important,
    Optional,
}

/// Represents a structural lobe in the Syntra Kernel ecosystem.
#[cfg_attr(feature = "agi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct EcosystemLobe {
    pub name: String,
    pub path: PathBuf,
    pub present: bool,
    pub size_bytes: u64,
    pub is_empty: bool,
    pub file_count: u64,
    pub kind: LobeKind,
    pub criticality: Criticality,
    pub tags: Vec<String>,
    pub completeness: f32,
}

/// High-level model of Syntra Kernel’s ecosystem health.
#[cfg_attr(feature = "agi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct EcosystemModel {
    pub lobes: Vec<EcosystemLobe>,
    pub missing: Vec<String>,
    pub incomplete: Vec<String>,
    pub upgrade_recommendations: Vec<String>,
    pub health_score: f32,

    #[cfg_attr(feature = "agi", serde(skip))]
    pub telemetry: Option<TelemetryBus>,
}

impl EcosystemModel {
    pub fn new() -> Self {
        Self {
            lobes: Vec::new(),
            missing: Vec::new(),
            incomplete: Vec::new(),
            upgrade_recommendations: Vec::new(),
            health_score: 0.0,
            telemetry: None,
        }
    }

    pub fn with_telemetry(telemetry: TelemetryBus) -> Self {
        Self {
            lobes: Vec::new(),
            missing: Vec::new(),
            incomplete: Vec::new(),
            upgrade_recommendations: Vec::new(),
            health_score: 0.0,
            telemetry: Some(telemetry),
        }
    }

    /// Scan the repository root and populate the ecosystem model.
    pub fn scan_repo(&mut self, root: impl AsRef<Path>) {
        let root = root.as_ref();

        if let Some(t) = &self.telemetry {
            t.log(
                TelemetryLevel::Info,
                format!("Ecosystem scan started at root: {}", root.display()),
            );
        }

        self.lobes.clear();
        self.missing.clear();
        self.incomplete.clear();
        self.upgrade_recommendations.clear();
        self.health_score = 0.0;

        let expected_lobes: Vec<(String, &str, LobeKind, Criticality, Vec<String>)> = vec![
            ("AGI Core".into(), "src/agi_core", LobeKind::Core, Criticality::Essential, vec!["agi".into(), "core".into()]),
            ("Cortex".into(), "src/cortex", LobeKind::Core, Criticality::Essential, vec!["cortex".into(), "introspection".into()]),
            ("Runtime".into(), "src/runtime", LobeKind::Runtime, Criticality::Essential, vec!["actors".into(), "scheduler".into()]),
            ("Renderer".into(), "src/renderer", LobeKind::Interface, Criticality::Important, vec!["gpu".into(), "viewport".into()]),
            ("Browser".into(), "src/browser", LobeKind::Interface, Criticality::Important, vec!["session".into(), "ui".into()]),
            ("Terminal".into(), "src/terminal", LobeKind::Interface, Criticality::Important, vec!["cli".into(), "shell".into()]),
            ("Utilities".into(), "src/utilities", LobeKind::Core, Criticality::Important, vec!["logging".into(), "helpers".into()]),
            ("Compiler Pipeline".into(), "src/parser", LobeKind::Compiler, Criticality::Important, vec!["lexer".into(), "ast".into(), "types".into()]),
            ("Dataset".into(), "src/dataset", LobeKind::Data, Criticality::Optional, vec!["storage".into(), "cache".into()]),
            ("Docs".into(), "docs", LobeKind::Docs, Criticality::Important, vec!["documentation".into()]),
        ];

        for (name, rel_path, kind, criticality, tags) in expected_lobes {
            let full_path = root.join(rel_path);

            if !full_path.exists() {
                self.missing.push(name.clone());
                self.lobes.push(EcosystemLobe {
                    name: name.clone(),
                    path: full_path.clone(),
                    present: false,
                    size_bytes: 0,
                    is_empty: true,
                    file_count: 0,
                    kind: kind.clone(),
                    criticality: criticality.clone(),
                    tags: tags.clone(),
                    completeness: 0.0,
                });

                if let Some(t) = &self.telemetry {
                    t.log(
                        TelemetryLevel::Warn,
                        format!("Ecosystem lobe missing: '{}' at '{}'", name, full_path.display()),
                    );
                }

                continue;
            }

            let (size_bytes, file_count) = scan_dir_stats(&full_path);
            let is_empty = size_bytes == 0 || file_count == 0;
            if is_empty {
                self.incomplete.push(name.clone());
            }

            let completeness = compute_completeness(&kind, &criticality, size_bytes, file_count);

            let lobe = EcosystemLobe {
                name: name.clone(),
                path: full_path.clone(),
                present: true,
                size_bytes,
                is_empty,
                file_count,
                kind: kind.clone(),
                criticality: criticality.clone(),
                tags: tags.clone(),
                completeness,
            };

            if let Some(t) = &self.telemetry {
                t.record(TelemetryEvent::Routing {
                    target: name.clone(),
                    confidence: completeness,
                    reason: format!(
                        "Ecosystem lobe completeness (files={}, size={}B)",
                        file_count, size_bytes
                    ),
                });

                if is_empty || completeness < 0.4 {
                    t.log(
                        TelemetryLevel::Warn,
                        format!(
                            "Ecosystem lobe '{}' appears incomplete (completeness {:.2}, files={}, size={}B)",
                            name, completeness, file_count, size_bytes
                        ),
                    );
                } else {
                    t.log(
                        TelemetryLevel::Info,
                        format!(
                            "Ecosystem lobe '{}' scanned: completeness {:.2}, files={}, size={}B",
                            name, completeness, file_count, size_bytes
                        ),
                    );
                }
            }

            self.lobes.push(lobe);
        }

        self.generate_recommendations();
        self.compute_health_score();

        if let Some(t) = &self.telemetry {
            let total = self.lobes.len();
            let blocked = self.missing.len() + self.incomplete.len();
            let allowed = total.saturating_sub(blocked);

            t.record(TelemetryEvent::EvolutionSummary { total, allowed, blocked });

            t.log(
                TelemetryLevel::Info,
                format!(
                    "Ecosystem scan complete. Health score={:.2}, missing={}, incomplete={}",
                    self.health_score,
                    self.missing.len(),
                    self.incomplete.len()
                ),
            );
        }
    }

    fn generate_recommendations(&mut self) {
        for lobe in &self.lobes {
            if !lobe.present {
                self.upgrade_recommendations.push(format!(
                    "Lobe '{}' is missing. Recommend generating a scaffold module with Rust \
                     skeletons, documentation, and integration tests.",
                    lobe.name
                ));
            } else if lobe.is_empty || lobe.completeness < 0.4 {
                self.upgrade_recommendations.push(format!(
                    "Lobe '{}' exists but appears incomplete (completeness {:.2}). \
                     Recommend adding core logic, documentation, and tests.",
                    lobe.name, lobe.completeness
                ));
            }
        }

        if self.missing.is_empty() && self.incomplete.is_empty() {
            self.upgrade_recommendations.push(
                "All lobes present and non-empty. Recommend evolutionary improvements: \
                 refactor AGI Core, optimize renderer, expand Cortex introspection."
                    .to_string(),
            );
        }
    }

    fn compute_health_score(&mut self) {
        if self.lobes.is_empty() {
            self.health_score = 0.0;
            return;
        }

        let mut weighted_sum = 0.0;
        let mut weight_total = 0.0;

        for lobe in &self.lobes {
            let weight = match lobe.criticality {
                Criticality::Essential => 3.0,
                Criticality::Important => 2.0,
                Criticality::Optional => 1.0,
            };
            weighted_sum += lobe.completeness * weight;
            weight_total += weight;
        }

        self.health_score = if weight_total > 0.0 {
            (weighted_sum / weight_total).clamp(0.0, 1.0)
        } else {
            0.0
        };
    }

    pub fn diagnostic_summary(&self) -> String {
        let mut out = String::new();

        out.push_str("=== Syntra Kernel Ecosystem Diagnostic ===\n");
        out.push_str(&format!("Overall health score: {:.2}\n\n", self.health_score));

        if !self.missing.is_empty() {
            out.push_str("Missing Lobes:\n");
            for m in &self.missing {
                out.push_str(&format!("  • {}\n", m));
            }
            out.push('\n');
        }

        if !self.incomplete.is_empty() {
            out.push_str("Incomplete Lobes:\n");
            for i in &self.incomplete {
                out.push_str(&format!("  • {}\n", i));
            }
            out.push('\n');
        }

        out.push_str("Lobe Completeness:\n");
        for l in &self.lobes {
            out.push_str(&format!(
                "  • {:<18} present={} files={} size={}B completeness={:.2}\n",
                l.name, l.present, l.file_count, l.size_bytes, l.completeness
            ));
        }
        out.push('\n');

        out.push_str("Recommendations:\n");
        for r in &self.upgrade_recommendations {
            out.push_str(&format!("  • {}\n", r));
        }

        out
    }
}

/// Recursively compute total size and file count for a directory.
fn scan_dir_stats(path: &Path) -> (u64, u64) {
    let mut size_bytes = 0u64;
    let mut file_count = 0u64;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() {
                    file_count += 1;
                    size_bytes += meta.len();
                } else if meta.is_dir() {
                    let (sub_size, sub_count) = scan_dir_stats(&p);
                    size_bytes += sub_size;
                    file_count += sub_count;
                }
            }
        }
    }

    (size_bytes, file_count)
}

/// Heuristically compute a completeness score for a lobe.
fn compute_completeness(
    kind: &LobeKind,
    criticality: &Criticality,
    size_bytes: u64,
    file_count: u64,
) -> f32 {
    if size_bytes == 0 || file_count == 0 {
        return 0.0;
    }

    let base = match kind {
        LobeKind::Core | LobeKind::Runtime | LobeKind::Compiler => 0.7,
        LobeKind::Interface | LobeKind::Docs => 0.6,
        LobeKind::Data | LobeKind::Experimental => 0.5,
    };

    let crit_boost = match criticality {
        Criticality::Essential => 0.15,
        Criticality::Important => 0.1,
        Criticality::Optional => 0.05,
    };

    let size_factor = ((size_bytes as f32).ln() / 14.0).clamp(0.0, 0.4);
    let file_factor = ((file_count as f32).ln() / 6.0).clamp(0.0, 0.4);

    (base + crit_boost + size_factor + file_factor).clamp(0.0, 1.0)
}

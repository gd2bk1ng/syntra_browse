/* ================================================================================================
   SYNTRA BROWSER — AXIOM FIVE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/agi_core/ecosystem.rs
   Module:      AGI Core — Ecosystem Model + Diagnostic Scanner
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Structural model of Syntra's ecosystem. Scans the repository, identifies missing
                lobes, incomplete modules, and recommends upgrades. Non-destructive and advisory.
   ================================================================================================ */

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

/// Represents a structural lobe in Syntra's ecosystem.
///
/// Lobes correspond to major filesystem or logical components such as source code,
/// documentation, or terminal interfaces.
#[derive(Debug, Clone)]
pub struct EcosystemLobe {
    /// Name of the lobe (e.g., "AGI Core", "Docs").
    pub name: String,
    /// Filesystem path or identifier for the lobe.
    pub path: PathBuf,
    /// Whether the lobe currently exists or is accessible.
    pub present: bool,
    /// Total size in bytes (0 if missing or unreadable).
    pub size_bytes: u64,
    /// Whether the lobe appears empty (size == 0).
    pub is_empty: bool,
}

/// High-level model of Syntra's ecosystem health.
///
/// This is read-only diagnostic state: it never mutates the filesystem.
#[derive(Debug, Clone, Default)]
pub struct EcosystemModel {
    /// Collection of ecosystem lobes representing the system structure.
    pub lobes: Vec<EcosystemLobe>,
    /// Names of lobes that are missing.
    pub missing: Vec<String>,
    /// Names of lobes that exist but appear empty/incomplete.
    pub incomplete: Vec<String>,
    /// Human-readable upgrade recommendations.
    pub upgrade_recommendations: Vec<String>,
}

impl EcosystemModel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Scan the repository and populate the ecosystem model.
    /// This function is intentionally non-destructive.
    pub fn scan_repo(&mut self, root: impl AsRef<Path>) {
        let root = root.as_ref();

        let expected_lobes = vec![
            ("AGI Core", "src/agi_core"),
            ("Browser", "src/browser"),
            ("Renderer", "src/browser/renderer"),
            ("Cortex", "src/cortex"),
            ("Terminal", "src/terminal"),
            ("Docs", "docs"),
        ];

        for (name, rel_path) in expected_lobes {
            let full_path = root.join(rel_path);

            if !full_path.exists() {
                self.missing.push(name.to_string());
                self.lobes.push(EcosystemLobe {
                    name: name.to_string(),
                    path: full_path.clone(),
                    present: false,
                    size_bytes: 0,
                    is_empty: true,
                });
                continue;
            }

            let (size, is_empty) = match fs::metadata(&full_path) {
                Ok(meta) => {
                    let s = meta.len();
                    (s, s == 0)
                }
                Err(_) => (0, true),
            };

            if is_empty {
                self.incomplete.push(name.to_string());
            }

            self.lobes.push(EcosystemLobe {
                name: name.to_string(),
                path: full_path.clone(),
                present: true,
                size_bytes: size,
                is_empty,
            });
        }

        self.generate_recommendations();
    }

    /// Generate upgrade recommendations based on missing/incomplete lobes.
    fn generate_recommendations(&mut self) {
        for missing in &self.missing {
            self.upgrade_recommendations.push(format!(
                "Lobe '{}' is missing. Recommend generating a scaffold module with Rust skeletons, \
                 tests, and documentation.",
                missing
            ));
        }

        for incomplete in &self.incomplete {
            self.upgrade_recommendations.push(format!(
                "Lobe '{}' exists but is empty or incomplete. Recommend adding core logic, \
                 documentation, and integration tests.",
                incomplete
            ));
        }

        if self.missing.is_empty() && self.incomplete.is_empty() {
            self.upgrade_recommendations.push(
                "All lobes present and non-empty. Recommend evolutionary improvements: \
                 refactor AGI Core, optimize renderer, expand Cortex introspection."
                    .to_string(),
            );
        }
    }

    /// Pretty-print a diagnostic summary.
    pub fn diagnostic_summary(&self) -> String {
        let mut out = String::new();

        out.push_str("=== Syntra Ecosystem Diagnostic ===\n");

        if !self.missing.is_empty() {
            out.push_str("Missing Lobes:\n");
            for m in &self.missing {
                out.push_str(&format!("  • {}\n", m));
            }
        }

        if !self.incomplete.is_empty() {
            out.push_str("Incomplete Lobes:\n");
            for i in &self.incomplete {
                out.push_str(&format!("  • {}\n", i));
            }
        }

        out.push_str("Recommendations:\n");
        for r in &self.upgrade_recommendations {
            out.push_str(&format!("  • {}\n", r));
        }

        out
    }
}

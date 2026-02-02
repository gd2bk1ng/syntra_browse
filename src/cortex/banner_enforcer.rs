// ================================================================================================
//   SYNTRA KERNEL — CORTEX (BANNER ENFORCER LOBE)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/cortex/banner_enforcer.rs
//   Module:      Banner Enforcer Lobe
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Scans the Syntra Kernel ecosystem, analyzes Rust files, and enforces canonical
//                banners using BannerLobe + CodeIntrospector. Supports dry-run diagnostics and
//                automatic rewriting.
// ================================================================================================

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

use crate::cortex::banner_lobe::BannerLobe;
use crate::cortex::code_introspector::{CodeIntrospector, FileAnalysis};

/// Result of enforcing a banner on a single file.
#[cfg_attr(feature = "agi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct BannerEnforcementResult {
    pub path: PathBuf,
    pub had_banner: bool,
    pub updated: bool,
    pub module_name: String,
    pub description: String,
}

/// High-level summary of a banner enforcement run.
#[cfg_attr(feature = "agi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct BannerEnforcementSummary {
    pub scanned_files: usize,
    pub updated_files: usize,
    pub skipped_files: usize,
    pub results: Vec<BannerEnforcementResult>,
}

pub struct BannerEnforcer {
    banner_lobe: BannerLobe,
    introspector: CodeIntrospector,
}

impl BannerEnforcer {
    pub fn new(banner_lobe: BannerLobe) -> Self {
        Self {
            banner_lobe,
            introspector: CodeIntrospector::new(),
        }
    }

    /// Recursively scan a root directory for `.rs` files and enforce banners.
    pub fn enforce_in_tree(
        &self,
        root: impl AsRef<Path>,
        dry_run: bool,
    ) -> std::io::Result<BannerEnforcementSummary> {
        let root = root.as_ref();
        let mut results = Vec::new();
        let mut scanned = 0usize;
        let mut updated = 0usize;
        let mut skipped = 0usize;

        for path in walk_rs_files(root)? {
            scanned += 1;

            let content = fs::read_to_string(&path)?;
            let analysis = self.introspector.analyze_content(&path, &content);

            let file_str = path.to_string_lossy();
            let result = self.enforce_for_file(&path, &content, &analysis, dry_run)?;

            if result.updated {
                updated += 1;
            } else {
                skipped += 1;
            }

            results.push(result);

            // Optional: progress logging could go here.
            let _ = file_str;
        }

        Ok(BannerEnforcementSummary {
            scanned_files: scanned,
            updated_files: updated,
            skipped_files: skipped,
            results,
        })
    }

    /// Enforce banner for a single file.
    pub fn enforce_for_file(
        &self,
        path: &Path,
        original_content: &str,
        analysis: &FileAnalysis,
        dry_run: bool,
    ) -> std::io::Result<BannerEnforcementResult> {
        let file_str = path.to_string_lossy();
        let module_name = analysis.module_name.clone();
        let description = analysis.short_description();

        let had_banner = self.banner_lobe.has_banner(original_content);
        let new_content = self
            .banner_lobe
            .apply_banner_to_content(&file_str, &module_name, &description, original_content);

        let updated = new_content != original_content;

        if updated && !dry_run {
            fs::write(path, new_content)?;
        }

        Ok(BannerEnforcementResult {
            path: path.to_path_buf(),
            had_banner,
            updated,
            module_name,
            description,
        })
    }
}

fn walk_rs_files(root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    walk_rs_files_inner(root, &mut out)?;
    Ok(out)
}

fn walk_rs_files_inner(root: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if root.is_dir() {
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_rs_files_inner(&path, out)?;
            } else if path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "rs")
                .unwrap_or(false)
            {
                out.push(path);
            }
        }
    }
    Ok(())
}

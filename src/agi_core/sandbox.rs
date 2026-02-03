// ================================================================================================
//   SYNTRA KERNEL — SANDBOX EXECUTION ENGINE
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/sandbox.rs
//   Module:      AGI Core — Sandbox Execution Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Provides an isolated execution environment where Syntra can experiment on her own
//       codebase safely. The sandbox engine is responsible for:
//
//         • Cloning the current codebase into a sandbox directory
//         • Applying proposed self-modification changes inside the sandbox
//         • Running builds/tests/linters inside the sandbox
//         • Capturing logs, errors, and diffs
//         • Reporting results back to the core for human review
//
//       This is the core of Syntra’s self-healing and self-evolving loop.
//
//   Overview:
//       - SandboxConfig       — configuration for sandbox runs.
//       - SandboxResult       — outcome of a sandbox execution.
//       - SandboxEngine       — orchestrates sandbox lifecycle.
//       - run_with_plan()     — run a single plan.
//       - run_with_plans()    — run multiple plans sequentially.
//       - run_build()         — run `cargo build` in sandbox.
//       - run_tests()         — run `cargo test` in sandbox.
//
//   Notes:
//       - All mutations happen in the sandbox, never in the live tree.
//       - Live changes are only applied via SelfModEngine after human approval.
//       - MIT & Apache 2.0 dual-licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use crate::agi_core::{
    EvolutionPlan,
    SelfModEngine,
    SelfModPolicy,
};
use crate::utilities::{
    baseline_snapshot::BaselineSnapshot,
    fs_utils::{copy_tree, ensure_dir},
    path_utils::normalize_path,
};

use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use uuid::Uuid;

/// Configuration for a sandbox run.
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Root of the live repository.
    pub live_root: PathBuf,
    /// Root directory where sandboxes are created.
    pub sandbox_root: PathBuf,
    /// Whether to run `cargo build`.
    pub run_build: bool,
    /// Whether to run `cargo test`.
    pub run_tests: bool,
}

/// Result of a sandbox execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxResult {
    pub sandbox_path: String,
    pub plan_label: String,
    pub attempt: u32,
    pub build_ok: bool,
    pub test_ok: bool,
    pub build_output: String,
    pub test_output: String,
    pub diff_summary: String,
}

/// Sandbox engine orchestrates sandbox lifecycle.
pub struct SandboxEngine {
    config: SandboxConfig,
    policy: SelfModPolicy,
}

impl SandboxEngine {
    pub fn new(config: SandboxConfig, policy: SelfModPolicy) -> Self {
        Self { config, policy }
    }

    /// Run a single evolution plan in a fresh sandbox.
    ///
    /// Flow:
    ///   - clone live tree
    ///   - take baseline snapshot
    ///   - apply evolution plan in sandbox
    ///   - run build/tests
    ///   - compute diff
    ///   - return structured result
    pub fn run_with_plan(
        &self,
        plan: &EvolutionPlan,
        plan_label: &str,
        attempt: u32,
    ) -> anyhow::Result<SandboxResult> {
        let sandbox_dir = self.create_sandbox_dir(plan_label, attempt)?;

        // 1) Copy live tree into sandbox
        copy_tree(&self.config.live_root, &sandbox_dir)?;

        // 2) Take baseline snapshot (before changes)
        let before = BaselineSnapshot::scan_tree(&sandbox_dir, &self.policy)?;

        // 3) Apply plan inside sandbox
        let engine = SelfModEngine::new(sandbox_dir.clone(), self.policy.clone());
        engine.apply_plan(plan)?;

        // 4) Take snapshot after changes
        let after = BaselineSnapshot::scan_tree(&sandbox_dir, &self.policy)?;
        let diff = before.diff(&after);

        let diff_summary = format!(
            "Added: {}, Removed: {}, Modified: {}",
            diff.added.len(),
            diff.removed.len(),
            diff.modified.len()
        );

        // 5) Run build/tests if requested
        let (build_ok, build_output) = if self.config.run_build {
            self.run_build(&sandbox_dir)?
        } else {
            (true, String::from("build skipped"))
        };

        let (test_ok, test_output) = if self.config.run_tests {
            self.run_tests(&sandbox_dir)?
        } else {
            (true, String::from("tests skipped"))
        };

        Ok(SandboxResult {
            sandbox_path: normalize_path(&sandbox_dir),
            plan_label: plan_label.to_string(),
            attempt,
            build_ok,
            test_ok,
            build_output,
            test_output,
            diff_summary,
        })
    }

    /// Run multiple labeled plans sequentially and return all results.
    pub fn run_with_plans(
        &self,
        plans: &[(String, EvolutionPlan)],
    ) -> anyhow::Result<Vec<SandboxResult>> {
        let mut results = Vec::new();
        for (label, plan) in plans {
            let res = self.run_with_plan(plan, label, 1)?;
            results.push(res);
        }
        Ok(results)
    }

    /// Create a unique sandbox directory for a given plan/attempt.
    fn create_sandbox_dir(&self, label: &str, attempt: u32) -> anyhow::Result<PathBuf> {
        ensure_dir(&self.config.sandbox_root)?;
        let id = Uuid::new_v4().to_string();
        let dir = self
            .config
            .sandbox_root
            .join(format!("{}_attempt{}_{}", label, attempt, id));
        ensure_dir(&dir)?;
        Ok(dir)
    }

    fn run_build(&self, sandbox_dir: &Path) -> anyhow::Result<(bool, String)> {
        let output = Command::new("cargo")
            .arg("build")
            .current_dir(sandbox_dir)
            .output()?;
        Ok(process_output(output))
    }

    fn run_tests(&self, sandbox_dir: &Path) -> anyhow::Result<(bool, String)> {
        let output = Command::new("cargo")
            .arg("test")
            .current_dir(sandbox_dir)
            .output()?;
        Ok(process_output(output))
    }
}

fn process_output(output: Output) -> (bool, String) {
    let ok = output.status.success();
    let mut buf = String::new();

    if !output.stdout.is_empty() {
        buf.push_str(&String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        if !buf.is_empty() {
            buf.push_str("\n--- stderr ---\n");
        }
        buf.push_str(&String::from_utf8_lossy(&output.stderr));
    }

    (ok, buf)
}

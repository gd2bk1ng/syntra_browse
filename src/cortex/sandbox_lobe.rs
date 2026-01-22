/* ================================================================================================
   SYNTRA BROWSER — AXIOM FOUR
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/cortex/sandbox_lobe.rs
   Module:      Cortex — Sandbox Lobe (Self‑Modification Workspace)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Provides a virtual, in‑memory sandbox where Syntra can propose, stage, and review
                code changes without touching the real filesystem. This is Syntra’s “safe lab”
                for self‑modification experiments in Axiom Four.

   Overview:
     • SandboxFile      — Represents a virtual file (path + content).
     • SandboxPatch     — Represents a proposed modification to a file.
     • SandboxSession   — In‑memory workspace with files and patches.
     • apply_patch()    — Applies a patch to a virtual file.
     • diff()           — Produces a human‑readable diff for review.
     • snapshot()       — Returns a snapshot of the current sandbox state.

   Notes:
     - Axiom Four never writes directly to disk from this lobe.
     - All changes are proposals, subject to human review and external tooling.
     - Future axioms may integrate static analysis, tests, and auto‑approval policies.
   ================================================================================================ */

#![allow(dead_code)]

use crate::utilities::{trace_enter, trace_exit};

#[derive(Debug, Clone)]
pub struct SandboxFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct SandboxPatch {
    pub path: String,
    pub original_snippet: String,
    pub replacement_snippet: String,
}

#[derive(Debug, Default)]
pub struct SandboxSession {
    files: Vec<SandboxFile>,
    patches: Vec<SandboxPatch>,
}

impl SandboxSession {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            patches: Vec::new(),
        }
    }

    /// Load a file into the sandbox (virtual only).
    pub fn load_file(&mut self, path: &str, content: &str) {
        self.files.push(SandboxFile {
            path: path.to_string(),
            content: content.to_string(),
        });
    }

    /// Propose a patch to a file.
    pub fn propose_patch(&mut self, patch: SandboxPatch) {
        self.patches.push(patch);
    }

    /// Apply all patches to the in‑memory files.
    pub fn apply_patches(&mut self) {
        trace_enter("SandboxSession::apply_patches");

        for patch in &self.patches {
            if let Some(file) = self.files.iter_mut().find(|f| f.path == patch.path) {
                file.content = file
                    .content
                    .replace(&patch.original_snippet, &patch.replacement_snippet);
            }
        }

        trace_exit("SandboxSession::apply_patches");
    }

    /// Produce a simple diff view of all patches.
    pub fn diff(&self) -> String {
        let mut out = String::new();
        for patch in &self.patches {
            out.push_str(&format!("File: {}\n", patch.path));
            out.push_str("  --- original ---\n");
            out.push_str(&indent_block(&patch.original_snippet, "  "));
            out.push_str("  +++ proposed ---\n");
            out.push_str(&indent_block(&patch.replacement_snippet, "  "));
            out.push_str("\n");
        }
        if out.is_empty() {
            "No patches proposed in sandbox.".to_string()
        } else {
            out
        }
    }

    /// Snapshot of all sandbox files (for external tools or export).
    pub fn snapshot(&self) -> Vec<SandboxFile> {
        self.files.clone()
    }
}

fn indent_block(text: &str, prefix: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        out.push_str(prefix);
        out.push_str(line);
        out.push('\n');
    }
    out
}

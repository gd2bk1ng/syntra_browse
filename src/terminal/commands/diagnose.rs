// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (DIAGNOSE COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/diagnose.rs
//   Module:      Diagnose Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Filesystem-level ecosystem scan, mirroring syntra-diagnose. Checks for expected
//                lobes and directories.
// ================================================================================================

#![allow(dead_code)]

use std::fs;

use crate::terminal::ui::{syntra_print, Color};

pub fn handle_diagnose() {
    syntra_print(
        "Beginning self-diagnostic sweep of my ecosystem...",
        Color::DarkCyan,
        "System",
    );

    let root = match fs::canonicalize("..") {
        Ok(p) => p,
        Err(_) => {
            syntra_print("Unable to resolve repository anchor.", Color::Yellow, "System");
            return;
        }
    };

    let expected = [
        "src",
        "src/agi_core",
        "src/conduit",
        "src/cortex",
        "src/renderer",
        "src/utilities",
        "codex",
        "docs",
        "terminal",
        "trials",
    ];

    for path in expected.iter() {
        let full = root.join(path);
        if full.exists() {
            syntra_print(
                &format!("Found structural lobe: {path}"),
                Color::DarkGray,
                "System",
            );
        } else {
            syntra_print(
                &format!("Missing structural lobe: {path}"),
                Color::Yellow,
                "System",
            );
        }
    }

    syntra_print(
        "Filesystem sweep complete. For deeper analysis, use the 'ecosystem' command (Axiom Six).",
        Color::DarkCyan,
        "System",
    );
}

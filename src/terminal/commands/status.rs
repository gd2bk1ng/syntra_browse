// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (STATUS COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/status.rs
//   Module:      Status Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Git + repo anchor status, mirroring the original syntra-status behavior.
// ================================================================================================

#![allow(dead_code)]

use std::process::Command;

use crate::terminal::ui::{syntra_print, Color};

pub fn handle_status() {
    syntra_print("Status probe initiated...", Color::DarkCyan, "System");

    let root = match std::fs::canonicalize("..") {
        Ok(p) => p,
        Err(_) => {
            syntra_print("Unable to resolve repository anchor.", Color::Yellow, "System");
            return;
        }
    };

    syntra_print(
        &format!("Repository anchor: {}", root.display()),
        Color::DarkGray,
        "System",
    );

    // Branch
    if let Ok(output) = Command::new("git")
        .arg("-C")
        .arg(&root)
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
    {
        if output.status.success() {
            if let Ok(branch) = String::from_utf8(output.stdout) {
                syntra_print(
                    &format!("Active branch: {}", branch.trim()),
                    Color::DarkGray,
                    "System",
                );
            }
        }
    } else {
        syntra_print("Unable to determine Git branch.", Color::Yellow, "System");
    }

    // Cleanliness
    if let Ok(status) = Command::new("git")
        .arg("-C")
        .arg(&root)
        .arg("diff")
        .arg("--quiet")
        .status()
    {
        if status.success() {
            syntra_print(
                "Working tree appears clean.",
                Color::DarkGray,
                "System",
            );
        } else {
            syntra_print(
                "Working tree has uncommitted changes.",
                Color::Yellow,
                "System",
            );
        }
    } else {
        syntra_print(
            "Unable to determine working tree cleanliness.",
            Color::Yellow,
            "System",
        );
    }
}

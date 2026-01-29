// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (SYNC COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/sync.rs
//   Module:      Sync Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Git fast-forward sync with upstream, mirroring syntra-sync.
// ================================================================================================

#![allow(dead_code)]

use std::process::Command;

use crate::terminal::ui::{syntra_print, Color};

pub fn handle_sync() {
    let root = match std::fs::canonicalize("..") {
        Ok(p) => p,
        Err(_) => {
            syntra_print("Unable to resolve repository anchor.", Color::Yellow, "System");
            return;
        }
    };

    syntra_print(
        "Contacting the upstream node and awaiting instructions.",
        Color::DarkCyan,
        "System",
    );
    syntra_print(
        "Initiating fast-forward sync with origin/axiom_*...",
        Color::DarkGray,
        "System",
    );

    match Command::new("git")
        .arg("-C")
        .arg(&root)
        .arg("pull")
        .arg("--ff-only")
        .status()
    {
        Ok(_status) => {
            // We could stream output, but for now we just report completion.
        }
        Err(_) => {
            syntra_print(
                "Synchronization encountered an error.",
                Color::Red,
                "System",
            );
        }
    }

    syntra_print(
        "Synchronization cycle complete.",
        Color::DarkCyan,
        "System",
    );
}

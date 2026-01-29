// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (INTERACTIVE CLI LOBE)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/mod.rs
//   Module:      Terminal — Interactive CLI Lobe
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Terminal/CLI front-end for Syntra Kernel. Provides commands for system diagnostics,
//                cognitive routing into the Cortex, evolution/sandbox inspection, and safety-aware
//                self-modification flows.
// ================================================================================================

#![allow(dead_code)]

pub mod ui;
pub mod cli;
pub mod repl;
pub mod commands;

pub use cli::run_cli;
pub use repl::start_repl;
pub use ui::{print_banner, print_prompt, syntra_print};

/// Hint used by the Cortex stub run loop.
pub fn print_terminal_hint() {
    println!("💻 Syntra Terminal: run `syntra` or `syntra --help` from your shell.");
}

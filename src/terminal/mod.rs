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
//   Description: Terminal/CLI front-end for Syntra Kernel. Provides commands for banner enforcement,
//                code introspection, and future developer tools.
// ================================================================================================

#![allow(dead_code)]

pub mod cli;
pub mod commands;

pub use cli::run_cli;

pub fn print_terminal_hint() {
    println!("💻 Syntra Terminal: type `syntra --help` for available commands.");
}

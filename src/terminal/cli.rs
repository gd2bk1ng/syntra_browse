// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (CLI DISPATCHER)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/cli.rs
//   Module:      CLI Dispatcher
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Minimal CLI parser/dispatcher for Syntra Kernel terminal commands. Supports both
//                single-shot commands and an interactive REPL.
// ================================================================================================

#![allow(dead_code)]

use std::env;

use crate::agi_core::{NullReasoner, Reasoner};
use crate::conduit::Conduit;
use crate::cortex::Cortex;

use crate::terminal::commands;
use crate::terminal::repl::start_repl;
use crate::terminal::ui::{syntra_print, Color};

pub fn run_cli() {
    let args: Vec<String> = env::args().collect();

    // No args → interactive REPL.
    if args.len() <= 1 {
        let conduit = Conduit::new();
        let reasoner = NullReasoner::new();
        let mut cortex = Cortex::new(reasoner, conduit);
        start_repl(&mut cortex);
        return;
    }

    let cmd = args[1].as_str();
    let rest = &args[2..];

    let conduit = Conduit::new();
    let reasoner = NullReasoner::new();
    let mut cortex = Cortex::new(reasoner, conduit);

    match cmd {
        "help" | "--help" | "-h" => commands::help::handle_help(),
        "status" => commands::status::handle_status(),
        "diagnose" => commands::diagnose::handle_diagnose(),
        "self" => commands::self_check::handle_self_check(),
        "sync" => commands::sync::handle_sync(),
        "browse" => commands::browse::handle_browse(&mut cortex, rest),
        "knowledge" | "search" => commands::knowledge::handle_knowledge(&mut cortex, rest),
        "task" => commands::task::handle_task(&mut cortex, rest),
        "perceive" => commands::perceive::handle_perceive(&mut cortex, rest),
        "act" => commands::act::handle_act(&mut cortex, rest),
        "evolve" => commands::evolve::handle_evolve(&mut cortex, rest),
        "sandbox" => commands::sandbox::handle_sandbox(&mut cortex, rest),
        "thoughts" => commands::thoughts::handle_thoughts(&mut cortex),
        "ecosystem" => commands::ecosystem::handle_ecosystem(&mut cortex),
        "propose" => commands::propose::handle_propose(&mut cortex),
        "propose-safe" => commands::propose_safe::handle_propose_safe(&mut cortex),
        "safety" => commands::safety::handle_safety(&mut cortex),
        "banner" => commands::banner::handle_banner_command(&mut cortex, rest),
        "introspect" => commands::introspect::handle_introspect_command(&mut cortex, rest),
        other => {
            syntra_print(
                &format!("I received your intent: '{other}'."),
                Color::DarkGray,
                "Core",
            );
            syntra_print(
                "Routing this intent to my Syntra Kernel via the Cortex.",
                Color::DarkGray,
                "Bridge",
            );
            let response = cortex.process(other);
            syntra_print(&response, Color::White, "Cortex");
        }
    }
}

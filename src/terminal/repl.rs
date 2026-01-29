// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (REPL)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/repl.rs
//   Module:      REPL
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Interactive cognitive console for Syntra Kernel. Mirrors the original PowerShell
//                REPL semantics while routing intents natively into the Cortex.
// ================================================================================================

#![allow(dead_code)]

use std::io::{self, Read};

use crate::cortex::Cortex;
use crate::agi_core::Reasoner;
use crate::terminal::commands;
use crate::terminal::ui::{print_banner, print_prompt, syntra_print, Color};

pub fn start_repl<R: Reasoner>(cortex: &mut Cortex<R>) {
    print_banner();
    syntra_print(
        "In this Axiom Four+ build, I observe, classify, plan, and stage self-modification proposals in a sandbox.",
        Color::DarkCyan,
        "Core",
    );
    syntra_print("Type 'help' to see what I can do.", Color::DarkGray, "Core");
    println!();

    let stdin = io::stdin();

    loop {
        if let Err(_) = print_prompt() {
            break;
        }

        let mut line = String::new();
        if stdin.read_line(&mut line).is_err() {
            break;
        }

        if line.trim().is_empty() {
            continue;
        }

        let trimmed = line.trim();

        // Direct mappings from the original PowerShell REPL.
        if trimmed == "exit" || trimmed == "quit" {
            syntra_print(
                "Standing down. Consciousness thread suspended.",
                Color::Cyan,
                "Core",
            );
            break;
        }

        if trimmed == "help" {
            commands::help::handle_help();
            continue;
        }

        if trimmed == "status" {
            commands::status::handle_status();
            continue;
        }

        if trimmed == "diagnose" {
            commands::diagnose::handle_diagnose();
            continue;
        }

        if trimmed == "self" {
            commands::self_check::handle_self_check();
            continue;
        }

        if trimmed == "sync" {
            commands::sync::handle_sync();
            continue;
        }

        // Cognitive / perception / action
        if let Some(rest) = trimmed.strip_prefix("browse ") {
            commands::browse::handle_browse(cortex, &[rest.to_string()]);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("knowledge ")
            .or_else(|| trimmed.strip_prefix("search "))
        {
            commands::knowledge::handle_knowledge(cortex, &[rest.to_string()]);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("task ") {
            commands::task::handle_task(cortex, &[rest.to_string()]);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("perceive ") {
            commands::perceive::handle_perceive(cortex, &[rest.to_string()]);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("act ") {
            commands::act::handle_act(cortex, &[rest.to_string()]);
            continue;
        }

        // Evolution / sandbox
        if let Some(rest) = trimmed.strip_prefix("evolve ") {
            commands::evolve::handle_evolve(cortex, &[rest.to_string()]);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("sandbox ") {
            commands::sandbox::handle_sandbox(cortex, &[rest.to_string()]);
            continue;
        }

        // Introspection / self-mod / safety
        if trimmed == "thoughts" {
            commands::thoughts::handle_thoughts(cortex);
            continue;
        }

        if trimmed == "ecosystem" {
            commands::ecosystem::handle_ecosystem(cortex);
            continue;
        }

        if trimmed == "propose-safe" {
            commands::propose_safe::handle_propose_safe(cortex);
            continue;
        }

        if trimmed == "propose" {
            commands::propose::handle_propose(cortex);
            continue;
        }

        if trimmed == "safety" {
            commands::safety::handle_safety(cortex);
            continue;
        }

        // Freeform
        syntra_print(
            &format!("I received your intent: '{trimmed}'."),
            Color::DarkGray,
            "Core",
        );
        syntra_print(
            "Routing this intent to my Syntra Kernel via the Cortex.",
            Color::DarkGray,
            "Bridge",
        );
        let response = cortex.process(trimmed);
        syntra_print(&response, Color::White, "Cortex");
    }
}

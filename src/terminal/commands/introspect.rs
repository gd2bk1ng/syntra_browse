// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (INTROSPECT COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/introspect.rs
//   Module:      Introspect Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: CLI command to introspect a Rust source file via Cortex CodeIntrospector.
// ================================================================================================

#![allow(dead_code)]

use std::path::PathBuf;

use crate::agi_core::Reasoner;
use crate::cortex::Cortex;
use crate::terminal::ui::{syntra_print, Color};

pub fn handle_introspect_command<R: Reasoner>(cortex: &mut Cortex<R>, args: &[String]) {
    if args.is_empty() {
        syntra_print("Usage: introspect <file>", Color::Yellow, "Core");
        return;
    }

    let path = PathBuf::from(&args[0]);
    match cortex.introspect_file(&path) {
        Ok(analysis) => {
            syntra_print(
                &format!("Module: {}", analysis.module_name),
                Color::DarkGray,
                "Cortex",
            );
            syntra_print(
                &format!("Role:   {:?}", analysis.role),
                Color::DarkGray,
                "Cortex",
            );
            syntra_print(
                &format!("Description:\n  {}", analysis.description),
                Color::White,
                "Cortex",
            );

            if !analysis.notes.is_empty() {
                println!("Notes:");
                for n in analysis.notes {
                    println!("  - {n}");
                }
            }

            if !analysis.structs.is_empty() {
                println!("Structs: {}", analysis.structs.join(", "));
            }
            if !analysis.traits.is_empty() {
                println!("Traits:  {}", analysis.traits.join(", "));
            }
            if !analysis.functions.is_empty() {
                println!("Functions: {}", analysis.functions.join(", "));
            }
        }
        Err(e) => {
            syntra_print(
                &format!("Failed to introspect file: {e:?}"),
                Color::Red,
                "Core",
            );
        }
    }
}

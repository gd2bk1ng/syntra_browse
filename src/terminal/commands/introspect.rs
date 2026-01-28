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

use crate::cortex::Cortex;

pub fn handle_introspect_command<R>(cortex: &mut Cortex<R>, args: &[String])
where
    R: crate::agi_core::Reasoner,
{
    if args.is_empty() {
        eprintln!("Usage: introspect <file>");
        return;
    }

    let path = PathBuf::from(&args[0]);
    match cortex.introspect_file(&path) {
        Ok(analysis) => {
            println!("Module: {}", analysis.module_name);
            println!("Role:   {:?}", analysis.role);
            println!("Description:\n  {}", analysis.description);
            if !analysis.notes.is_empty() {
                println!("Notes:");
                for n in analysis.notes {
                    println!("  - {}", n);
                }
            }
            if !analysis.structs.is_empty() {
                println!("Structs: {}", analysis.structs.join(", "));
            }
            if !analysis.traits.is_empty() {
                println!("Traits:  {}", analysis.traits.join(", "));
            }
            if !analysis.functions.is_empty() {
                println!(
                    "Functions: {}",
                    analysis.functions.join(", ")
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to introspect file: {:?}", e);
        }
    }
}

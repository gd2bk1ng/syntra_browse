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
//   Description: Minimal CLI parser/dispatcher for Syntra Kernel terminal commands.
// ================================================================================================

#![allow(dead_code)]

use std::env;
use std::path::PathBuf;

use crate::cortex::Cortex;
use crate::agi_core::NullReasoner;
use crate::conduit::Conduit;

use crate::terminal::commands;

pub fn run_cli() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        commands::help::print_help();
        return;
    }

    let cmd = args[1].as_str();
    let mut cortex = Cortex::new(NullReasoner::new(), Conduit::new());

    match cmd {
        "banner" => commands::banner::handle_banner_command(&mut cortex, &args[2..]),
        "introspect" => commands::introspect::handle_introspect_command(&mut cortex, &args[2..]),
        "help" | "--help" | "-h" => commands::help::print_help(),
        _ => {
            eprintln!("Unknown command: {}", cmd);
            commands::help::print_help();
        }
    }
}

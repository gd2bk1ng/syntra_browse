// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (HELP COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/help.rs
//   Module:      Help Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Prints available Syntra Terminal commands.
// ================================================================================================

#![allow(dead_code)]

pub fn print_help() {
    println!("Syntra Kernel — Terminal");
    println!("Commands:");
    println!("  banner enforce <root> [--dry]       Enforce banners in tree");
    println!("  banner set-author <name>           Set banner author");
    println!("  introspect <file>                  Introspect a Rust source file");
    println!("  help                               Show this help");
}

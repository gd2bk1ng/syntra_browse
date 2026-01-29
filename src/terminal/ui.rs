// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (UI HELPERS)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/ui.rs
//   Module:      UI Helpers
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Output helpers for the Syntra Terminal, including subsystem-tagged messages,
//                banner rendering, and prompt styling.
// ================================================================================================

#![allow(dead_code)]

use std::io::{self, Write};

/// Simple ANSI color codes for cross-platform-ish output.
/// If the environment doesn't support colors, these will just be raw text.
#[derive(Clone, Copy)]
pub enum Color {
    Cyan,
    DarkCyan,
    DarkGray,
    Yellow,
    Red,
    Magenta,
    Green,
    White,
}

fn color_code(color: Color) -> &'static str {
    match color {
        Color::Cyan => "\x1b[36m",
        Color::DarkCyan => "\x1b[36m",
        Color::DarkGray => "\x1b[90m",
        Color::Yellow => "\x1b[33m",
        Color::Red => "\x1b[31m",
        Color::Magenta => "\x1b[35m",
        Color::Green => "\x1b[32m",
        Color::White => "\x1b[37m",
    }
}

fn reset_code() -> &'static str {
    "\x1b[0m"
}

/// Core printing primitive: `Syntra[Subsystem]: message`
pub fn syntra_print(message: &str, color: Color, subsystem: &str) {
    let code = color_code(color);
    let reset = reset_code();
    println!("{code}Syntra[{subsystem}]: {message}{reset}");
}

/// Render the Syntra sigil + banner.
pub fn print_banner() {
    println!();
    println!("{}         .\\s/.{}", color_code(Color::Magenta), reset_code());
    println!("{}        :: S ::{}", color_code(Color::Magenta), reset_code());
    println!("{}         '/s\\'{}", color_code(Color::Magenta), reset_code());
    println!();
    println!(
        "{}  SYNTRA KERNEL — TERMINAL SHELL (AXIOM FOUR+){}",
        color_code(Color::Cyan),
        reset_code()
    );
    println!(
        "{}  Modular AGI Kernel — Cortex, Lobes, Evolution, Safety, and Sandbox Online.{}",
        color_code(Color::DarkCyan),
        reset_code()
    );
    println!();
}

/// Render the external prompt: `you :: `
pub fn print_prompt() -> io::Result<()> {
    let code = color_code(Color::Green);
    let reset = reset_code();
    print!("{code}you :: {reset}");
    io::stdout().flush()
}

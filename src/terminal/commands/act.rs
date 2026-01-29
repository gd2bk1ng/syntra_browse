// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (ACT COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/act.rs
//   Module:      Act Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Axiom Four → Action lobe. Run a simple system command via the Cortex.
// ================================================================================================

#![allow(dead_code)]

use crate::cortex::Cortex;
use crate::agi_core::Reasoner;
use crate::terminal::ui::{syntra_print, Color};

pub fn handle_act<R: Reasoner>(cortex: &mut Cortex<R>, args: &[String]) {
    if args.is_empty() {
        syntra_print("Usage: act <cmd> [args...]", Color::Yellow, "Axiom4");
        return;
    }

    let cmd = args.join(" ");
    syntra_print(
        &format!("Intent: act {cmd}"),
        Color::DarkGray,
        "Axiom4",
    );
    syntra_print(
        "Engaging action lobe for system command.",
        Color::DarkGray,
        "Cortex",
    );

    let intent = format!("action {cmd}");
    let response = cortex.process(&intent);
    syntra_print(&response, Color::White, "Cortex");
}

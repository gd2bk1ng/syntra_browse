// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (PERCEIVE COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/perceive.rs
//   Module:      Perceive Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Axiom Four → Perception lobe. Run the perception lobe on arbitrary text.
// ================================================================================================

#![allow(dead_code)]

use crate::cortex::Cortex;
use crate::agi_core::Reasoner;
use crate::terminal::ui::{syntra_print, Color};

pub fn handle_perceive<R: Reasoner>(cortex: &mut Cortex<R>, args: &[String]) {
    if args.is_empty() {
        syntra_print("Usage: perceive <text>", Color::Yellow, "Axiom4");
        return;
    }

    let text = args.join(" ");
    syntra_print("Intent: perceive <text>", Color::DarkGray, "Axiom4");
    syntra_print(
        "Routing text to perception lobe.",
        Color::DarkGray,
        "Cortex",
    );

    let intent = format!("perception {text}");
    let response = cortex.process(&intent);
    syntra_print(&response, Color::White, "Cortex");
}

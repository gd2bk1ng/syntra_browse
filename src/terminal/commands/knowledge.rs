// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (KNOWLEDGE COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/knowledge.rs
//   Module:      Knowledge Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Axiom Four → Knowledge lobe. Search semantic memory via the Cortex.
// ================================================================================================

#![allow(dead_code)]

use crate::cortex::Cortex;
use crate::agi_core::Reasoner;
use crate::terminal::ui::{syntra_print, Color};

pub fn handle_knowledge<R: Reasoner>(cortex: &mut Cortex<R>, args: &[String]) {
    if args.is_empty() {
        syntra_print("Usage: knowledge <query>", Color::Yellow, "Axiom4");
        return;
    }

    let query = args.join(" ");
    syntra_print(
        &format!("Intent: knowledge {query}"),
        Color::DarkGray,
        "Axiom4",
    );
    syntra_print(
        "Querying knowledge lobe via kernel.",
        Color::DarkGray,
        "Cortex",
    );

    let intent = format!("knowledge {query}");
    let response = cortex.process(&intent);
    syntra_print(&response, Color::White, "Cortex");
}

// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (BROWSE COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/browse.rs
//   Module:      Browse Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Axiom Four → Perception + Knowledge lobes. Fetch a URL, perceive it, and store
//                it in knowledge via the Cortex.
// ================================================================================================

#![allow(dead_code)]

use crate::cortex::Cortex;
use crate::agi_core::Reasoner;
use crate::terminal::ui::{syntra_print, Color};

pub fn handle_browse<R: Reasoner>(cortex: &mut Cortex<R>, args: &[String]) {
    if args.is_empty() {
        syntra_print("Usage: browse <url>", Color::Yellow, "Axiom4");
        return;
    }

    let url = args.join(" ");
    syntra_print(
        &format!("Intent: browse {url}"),
        Color::DarkGray,
        "Axiom4",
    );
    syntra_print(
        "Activating perception + knowledge lobes via kernel.",
        Color::DarkGray,
        "Cortex",
    );

    let intent = format!("browse {url}");
    let response = cortex.process(&intent);
    syntra_print(&response, Color::White, "Cortex");
}

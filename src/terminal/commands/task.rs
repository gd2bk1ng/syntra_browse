// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (TASK COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/task.rs
//   Module:      Task Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Axiom Four → Execution lobe. Run a simple multi-step task via the Cortex.
// ================================================================================================

#![allow(dead_code)]

use crate::cortex::Cortex;
use crate::agi_core::Reasoner;
use crate::terminal::ui::{syntra_print, Color};

pub fn handle_task<R: Reasoner>(cortex: &mut Cortex<R>, args: &[String]) {
    if args.is_empty() {
        syntra_print("Usage: task <name>", Color::Yellow, "Axiom4");
        return;
    }

    let name = args.join(" ");
    syntra_print(
        &format!("Intent: task {name}"),
        Color::DarkGray,
        "Axiom4",
    );
    syntra_print(
        "Engaging execution lobe for multi-step task.",
        Color::DarkGray,
        "Cortex",
    );

    let intent = format!("task {name}");
    let response = cortex.process(&intent);
    syntra_print(&response, Color::White, "Cortex");
}

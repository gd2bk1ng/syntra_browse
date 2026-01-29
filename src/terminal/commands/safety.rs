// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (SAFETY COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/safety.rs
//   Module:      Safety Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Axiom Seven → SafetyPolicy. Queries the active safety policy and protected lobes.
// ================================================================================================

#![allow(dead_code)]

use crate::cortex::Cortex;
use crate::agi_core::Reasoner;
use crate::terminal::ui::{syntra_print, Color};

pub fn handle_safety<R: Reasoner>(cortex: &mut Cortex<R>) {
    syntra_print(
        "Querying active safety policy and protected lobes.",
        Color::DarkGray,
        "Axiom7",
    );

    let response = cortex.process("safety");
    syntra_print(&response, Color::White, "Cortex");
}

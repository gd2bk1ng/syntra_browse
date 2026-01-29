// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (PROPOSE COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/propose.rs
//   Module:      Propose Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Axiom Six → SelfModEngine. Runs the self-modification engine and shows an
//                evolution plan.
// ================================================================================================

#![allow(dead_code)]

use crate::cortex::Cortex;
use crate::agi_core::Reasoner;
use crate::terminal::ui::{syntra_print, Color};

pub fn handle_propose<R: Reasoner>(cortex: &mut Cortex<R>) {
    syntra_print(
        "Running self-modification engine and generating an evolution plan.",
        Color::DarkGray,
        "Axiom6",
    );

    let response = cortex.process("propose");
    syntra_print(&response, Color::White, "Cortex");
}

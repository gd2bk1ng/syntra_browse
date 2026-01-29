// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (ECOSYSTEM COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/ecosystem.rs
//   Module:      Ecosystem Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Axiom Six → EcosystemModel. Requests a detailed ecosystem diagnostic from the
//                kernel.
// ================================================================================================

#![allow(dead_code)]

use crate::cortex::Cortex;
use crate::agi_core::Reasoner;
use crate::terminal::ui::{syntra_print, Color};

pub fn handle_ecosystem<R: Reasoner>(cortex: &mut Cortex<R>) {
    syntra_print(
        "Requesting detailed ecosystem diagnostic from kernel (EcosystemModel).",
        Color::DarkGray,
        "Axiom6",
    );

    let response = cortex.process("ecosystem");
    syntra_print(&response, Color::White, "Cortex");
}

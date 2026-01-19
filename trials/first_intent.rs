/* ================================================================================================
   SYNTRA BROWSER — AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   File:        trials/first_intent.rs
   Module:      Trial — First Intent Pipeline Demo
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Experimental prototype demonstrating a minimal end‑to‑end intent pipeline using
                the AGI Core, Cortex, and Conduit modules.

   Notes:
     This is a laboratory file. It is not part of the production runtime. Use it to experiment
     with new cognitive flows and logging patterns.
   ================================================================================================ */

use syntra_browse::agi_core::{HeuristicReasoner, Intent};
use syntra_browse::conduit::{Conduit, ConduitMessage};
use syntra_browse::cortex::Cortex;
use syntra_browse::utilities::log_info;

/// Runs a minimal intent pipeline demo.
///
/// This function:
///  1. Creates a Conduit and Cortex with a HeuristicReasoner.
///  2. Sends a sample intent string into the Cortex.
///  3. Pumps messages from the Conduit and logs them.
pub fn run_trial() {
    log_info("Starting First Intent Trial…");

    let conduit = Conduit::new();
    let reasoner = HeuristicReasoner::default();
    let cortex = Cortex::new(reasoner, conduit);

    let raw_intent = "open: syntra://axiom_zero/overview";
    cortex.handle_intent(raw_intent);
    cortex.pump_messages();

    log_info("First Intent Trial complete.");
}

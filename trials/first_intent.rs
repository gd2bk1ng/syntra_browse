/* ================================================================================================
   SYNTRA KERNEL — AXIOM ZERO
   ------------------------------------------------------------------------------------------------
        .\s/.
       :: S ::
        '/s\'

   File:        trials/first_intent.rs
   Module:      Trial — First Intent Pipeline Demo
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Experimental prototype demonstrating a minimal end‑to‑end intent pipeline using
                the Syntra Kernel cognitive runtime. This trial showcases the Cortex, Conduit,
                Reasoner, and Intent subsystems working together in a simplified cognitive loop.

   Notes:
     This is a laboratory file. It is not part of the production runtime. Use it to experiment
     with new cognitive flows, message passing, and logging patterns. This file represents the
     earliest “Axiom Zero” intent pipeline and is intentionally minimal.
   ================================================================================================ */

use syntra_kernel::cortex::Cortex;
use syntra_kernel::conduit::{Conduit, ConduitMessage};
use syntra_kernel::reasoning::HeuristicReasoner;
use syntra_kernel::intent::Intent;
use syntra_kernel::utilities::log_info;

/// Runs a minimal intent pipeline demo.
///
/// This function:
///  1. Creates a Conduit and Cortex with a HeuristicReasoner.
///  2. Constructs a simple Intent object from a raw string.
///  3. Sends the intent into the Cortex.
///  4. Pumps messages from the Conduit and logs them.
///
/// This is the simplest possible demonstration of the Syntra Kernel cognitive loop.
pub fn run_trial() {
    log_info("Starting First Intent Trial…");

    // Create the message bus
    let conduit = Conduit::new();

    // Create the cognitive reasoner
    let reasoner = HeuristicReasoner::default();

    // Create the Cortex (central cognitive processor)
    let mut cortex = Cortex::new(reasoner, conduit.clone());

    // Example raw intent (Axiom Zero format)
    let raw_intent = "open syntra://axiom_zero/overview";

    // Convert raw string → structured Intent
    let intent = Intent::from_raw(raw_intent);

    // Feed the intent into the Cortex
    cortex.handle_intent(intent);

    // Pump messages from the Conduit
    while let Some(message) = conduit.try_recv() {
        match message {
            ConduitMessage::Log(text) => log_info(&format!("[Conduit] {text}")),
            ConduitMessage::Event(event) => {
                log_info(&format!("[Event] {:?}", event));
            }
            ConduitMessage::Error(err) => {
                log_info(&format!("[Error] {:?}", err));
            }
        }
    }

    log_info("First Intent Trial complete.");
}

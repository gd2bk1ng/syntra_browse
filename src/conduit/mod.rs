/* ================================================================================================
   SYNTRA BROWSER - AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/conduit/mod.rs
   Module:      Conduit (Inter-Module Communication)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Core message-passing infrastructure enabling decoupled communication between
                Syntra’s subsystems. The Conduit provides a lightweight, observable channel
                used by the AGI Core, Cortex, Renderer, Utilities, and external tools to
                exchange structured messages safely and predictably.

   Overview:
     • ConduitMessage — Typed messages exchanged between modules.
     • Conduit        — Lightweight communication channel (MPSC).
     • send           — Push messages into the channel.
     • try_recv       — Non-blocking message polling for event loops.

   Notes:
     - The Conduit acts as Syntra’s nervous system, enabling modular evolution without tight
       coupling between components.
     - This module is intentionally dependency-free for long-term stability.
     - Future expansions may include async channels, distributed conduits, or priority routing.
     - All message types remain ASCII-safe to ensure compatibility with external tools.
   ================================================================================================ */

#![allow(dead_code)]

pub mod bridge;

use std::sync::mpsc::{Sender, Receiver, channel};

/// A lightweight message type used for inter-module communication.
#[derive(Debug, Clone)]
pub enum ConduitMessage {
    /// Human-readable or system-generated log text.
    Log(String),

    /// A freeform intent string destined for the AGI core.
    Intent(String),

    /// Signals that the subsystem should shut down gracefully.
    Shutdown,
}

/// A simple conduit providing message passing between subsystems.
#[derive(Debug)]
pub struct Conduit {
    pub tx: Sender<ConduitMessage>,
    pub rx: Receiver<ConduitMessage>,
}

impl Conduit {
    /// Creates a new communication conduit.
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self { tx, rx }
    }

    /// Sends a message through the conduit.
    pub fn send(&self, msg: ConduitMessage) {
        let _ = self.tx.send(msg);
    }

    /// Attempts to receive a message without blocking.
    pub fn try_recv(&self) -> Option<ConduitMessage> {
        self.rx.try_recv().ok()
    }
}

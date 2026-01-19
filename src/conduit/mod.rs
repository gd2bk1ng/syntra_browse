/* ================================================================================================
   SYNTRA BROWSER — AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   File:        src/conduit/mod.rs
   Module:      Conduit (Inter‑Module Communication)
   Author:      Alexandr Roussinov
   Description: Message‑passing infrastructure for decoupled communication between Syntra’s
                subsystems. Provides a clean, observable channel for AGI, Cortex, Renderer, and
                Utilities to exchange structured messages.

   Overview:
     • ConduitMessage — Typed messages exchanged between modules.
     • Conduit        — Lightweight communication channel (MPSC).
     • try_recv       — Non‑blocking message polling.

   Notes:
     The Conduit is Syntra’s nervous system. Keep it simple, predictable, and observable.
   ================================================================================================ */

#![allow(dead_code)]

use std::sync::mpsc::{Sender, Receiver, channel};

/// A lightweight message type used for inter‑module communication.
#[derive(Debug, Clone)]
pub enum ConduitMessage {
    Log(String),
    Intent(String),
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

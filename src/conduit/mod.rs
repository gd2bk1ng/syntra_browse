/* ================================================================================================
   Syntra Browser — Axiom Zero
   Advanced AGI-Driven Intent Engine & Cognitive Rendering System
   ------------------------------------------------------------------------------------------------
   File:        src/conduit/mod.rs
   Module:      Conduit (Inter-Module Communication)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Created:     2026
   License:     MIT
   Repository:  https://github.com/gd2bk1ng/syntra_browse
   ------------------------------------------------------------------------------------------------
   Overview:
   The Conduit module provides the communication backbone for Syntra’s subsystems. It enables
   message passing, event routing, and decoupled interaction between AGI components, the renderer,
   and the cognitive cortex.

   Notes for Future Engineers (2050+):
   - Keep communication channels simple and predictable.
   - Avoid global state; the conduit should remain explicit and observable.
   - This module is the nervous system of Syntra — treat it with care.
   ================================================================================================ */

#![allow(dead_code)]

use std::sync::mpsc::{Sender, Receiver, channel};

/// A lightweight message type used for inter-module communication.
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

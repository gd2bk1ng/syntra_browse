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
   Description: Message-passing infrastructure for decoupled communication between Syntra's
                subsystems. Provides a clean, observable channel for AGI, Cortex, Renderer, and
                Utilities to exchange structured messages.

   Notes:
     - The Conduit acts as Syntra's nervous system.
     - This module remains dependency-free and ASCII-safe for long-term stability.
   ================================================================================================ */

#![allow(dead_code)]

pub mod bridge;

use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

/// A lightweight message type used for inter-module communication.
#[derive(Debug, Clone)]
pub enum ConduitMessage {
    /// Human-readable or system-generated log text.
    Log(String),

    /// A freeform intent string destined for the AGI Core.
    Intent(String),

    /// Request for a self-analysis or ecosystem scan.
    SelfAnalysis(String),

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

    /// Convenience wrapper for sending log messages.
    pub fn send_log(&self, text: impl Into<String>) {
        let _ = self.tx.send(ConduitMessage::Log(text.into()));
    }

    /// Convenience wrapper for sending intent messages.
    pub fn send_intent(&self, text: impl Into<String>) {
        let _ = self.tx.send(ConduitMessage::Intent(text.into()));
    }

    /// Convenience wrapper for sending self-analysis requests.
    pub fn send_self_analysis(&self, text: impl Into<String>) {
        let _ = self.tx.send(ConduitMessage::SelfAnalysis(text.into()));
    }

    /// Attempts to receive a message without blocking.
    pub fn try_recv(&self) -> Option<ConduitMessage> {
        match self.rx.try_recv() {
            Ok(msg) => Some(msg),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(ConduitMessage::Shutdown),
        }
    }

    /// Blocking receive for long-lived loops.
    pub fn recv_blocking(&self) -> Option<ConduitMessage> {
        self.rx.recv().ok()
    }

    /// Gracefully closes the conduit by sending a shutdown signal.
    pub fn close(&self) {
        let _ = self.tx.send(ConduitMessage::Shutdown);
    }
}

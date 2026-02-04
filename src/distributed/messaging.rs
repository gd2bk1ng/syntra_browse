// ================================================================================================
//   SYNTRA KERNEL — DISTRIBUTED MESSAGING
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/distributed/messaging.rs
//   Module:      Distributed — Messaging Layer
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Minimal in-process message bus for distributed Syntra nodes. This is the
//                abstraction layer that can later be swapped for real networking, IPC, or
//                federated transports.
// ================================================================================================

use std::collections::VecDeque;
use std::fmt;

/// Identifier for a node in the distributed system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "node-{}", self.0)
    }
}

/// A message between nodes.
#[derive(Debug, Clone)]
pub struct Message {
    pub from: NodeId,
    pub to: NodeId,
    pub payload: String,
}

/// Simple in-memory message bus.
#[derive(Debug, Default)]
pub struct MessageBus {
    queue: VecDeque<Message>,
}

impl MessageBus {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn send(&self, msg: Message) {
        // We use interior mutability via a Mutex in a real implementation; for now,
        // keep it single-threaded and mutable via &mut where needed.
        // To keep the API ergonomic, we’ll accept &self but require &mut when draining.
        // (Callers will typically hold the bus inside a Cluster.)
        unsafe {
            let ptr = self as *const _ as *mut MessageBus;
            (*ptr).queue.push_back(msg);
        }
    }

    pub fn drain(&mut self) -> Vec<Message> {
        self.queue.drain(..).collect()
    }
}

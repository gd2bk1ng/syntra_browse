// ================================================================================================
//   SYNTRA KERNEL :: DISTRIBUTED MESSAGING
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/distributed/messaging.rs
//   Module:      Distributed :: Messaging Layer
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Minimal in-process message bus for distributed Syntra nodes. This is the
//                abstraction layer that can later be swapped for real networking, IPC, or
//                federated transports.
// ================================================================================================

use std::collections::VecDeque;
use std::fmt;
use std::sync::Mutex;

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
    queue: Mutex<VecDeque<Message>>,
}

impl MessageBus {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn send(&self, msg: Message) { 
        self.queue.lock().expect("message bus poisoned").push_back(msg); 
    }

    pub fn drain(&mut self) -> Vec<Message> {
        self.queue
            .lock()
            .expect("message bus poisoned")
            .drain(..)
            .collect()
    }
}

pub type DistributedMessage = Message;

pub

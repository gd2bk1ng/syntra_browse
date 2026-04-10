// ================================================================================================
//   SYNTRA KERNEL :: DISTRIBUTED NODE
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/distributed/node.rs
//   Module:      Distributed :: Node Abstraction
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Represents a single Syntra node in a distributed cluster. Nodes can send and
//                receive messages via the messaging layer and participate in cluster operations.
// ================================================================================================

use std::collections::VecDeque;

pub use crate::distributed::messaging::NodeId;
use crate::distributed:messaging::{Message, MessageBus};

/// Local view of a node in the cluster.
pub struct Node {
    pub id: NodeId,
    inbox: VecDeque<Message>,
}

impl Node {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            inbox: VecDeque::new(),
        }
    }

    /// Enqueue an incoming message.
    pub fn receive(&mut self, msg: Message) {
        self.inbox.push_back(msg);
    }

    /// Process all pending messages using the provided handler.
    pub fn drain_inbox<F>(&mut self, mut handler: F)
    where
        F: FnMut(&Message),
    {
        while let Some(msg) = self.inbox.pop_front() {
            handler(&msg);
        }
    }

    /// Send a message via the shared message bus.
    pub fn send(&self, bus: &MessageBus, to: NodeId, payload: String) {
        let msg = Message {
            from: self.id,
            to,
            payload,
        };
        bus.send(msg);
    }
}


// ================================================================================================
//   SYNTRA KERNEL — DISTRIBUTED CLUSTER
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/distributed/cluster.rs
//   Module:      Distributed — Cluster Management
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Manages a set of nodes and coordinates message delivery via the messaging layer.
//                This is a simple in-process cluster model that can later be backed by real
//                networking or IPC.
// ================================================================================================

use std::collections::HashMap;

use crate::distributed::messaging::{Message, MessageBus, NodeId};
use crate::distributed::node::Node;

/// In-process cluster of nodes.
pub struct Cluster {
    pub bus: MessageBus,
    nodes: HashMap<NodeId, Node>,
}

impl Cluster {
    pub fn new() -> Self {
        Self {
            bus: MessageBus::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    /// Deliver all pending messages from the bus to their target nodes.
    pub fn pump_messages(&mut self) {
        let messages: Vec<Message> = self.bus.drain();
        for msg in messages {
            if let Some(node) = self.nodes.get_mut(&msg.to) {
                node.receive(msg);
            }
        }
    }
}

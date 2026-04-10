// ================================================================================================
//   SYNTRA KERNEL :: DISTRIBUTED RUNTIME
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/distributed/mod.rs
//   Module:      Syntra Kernel :: Distributed Runtime
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Multi-node orchestration, federated cognition, and distributed intent resolution.
//                Enables Syntra Kernel to operate across clusters, devices, and networks.
//
//   Notes:
//     - Future integration: Raft/consensus, distributed memory, federated learning.
// ================================================================================================

pub mod node;
pub mod cluster;
pub mod messaging;

pub use cluster::Cluster;
pub use messaging::DistributedMessage;
pub use node::NodeId as NodeIdentity;


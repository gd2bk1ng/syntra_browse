// ================================================================================================
//   SYNTRA KERNEL :: SIMULATION ENGINE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/simulation/mod.rs
//   Module:      Syntra Kernel :: Simulation Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Sandboxed world simulation, agent-based modeling, and environment dynamics.
//                Enables cognitive testing, scenario evaluation, and synthetic environments.
//
//   Notes:
//     - Future integration: physics engines, multi-agent reinforcement learning.
// ================================================================================================

pub mod world;
pub mod agent;
pub mod dynamics;

pub use agent::Agent as SimAgent;
pub use dynamics::Simulation as EnvironmentDynamics;
pub use world::World as SimWorld;

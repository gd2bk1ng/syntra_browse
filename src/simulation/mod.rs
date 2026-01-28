// ================================================================================================
//   SYNTRA KERNEL — SIMULATION ENGINE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/simulation/mod.rs
//   Module:      Syntra Kernel — Simulation Engine
//   Description: Sandboxed world simulation, agent-based modeling, and environment dynamics.
//                Enables cognitive testing, scenario evaluation, and synthetic environments.
//
//   Notes:
//     - Future integration: physics engines, multi-agent reinforcement learning.
// ================================================================================================

pub mod world;
pub mod agent;
pub mod dynamics;

pub use agent::SimAgent;
pub use dynamics::EnvironmentDynamics;
pub use world::SimWorld;

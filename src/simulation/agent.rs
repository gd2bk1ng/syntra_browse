// ================================================================================================
//   SYNTRA KERNEL — SIMULATION AGENT
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/simulation/agent.rs
//   Module:      Simulation — Agent Model
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Minimal agent abstraction for the simulation subsystem. Agents can observe the
//                world, decide on actions, and update their internal state.
// ================================================================================================

use std::fmt;

/// Identifier for an agent in the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentId(pub u64);

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "agent-{}", self.0)
    }
}

/// A simple action in the grid world.
#[derive(Debug, Clone, Copy)]
pub enum AgentAction {
    Stay,
    Move { dx: i32, dy: i32 },
}

/// Trait for simulation agents.
pub trait Agent: Send {
    fn id(&self) -> AgentId;

    /// Decide an action given a tick index.
    fn decide(&mut self, tick: u64) -> AgentAction;
}

/// A trivial agent that walks in a fixed pattern.
pub struct WalkerAgent {
    id: AgentId,
    step: i32,
}

impl WalkerAgent {
    pub fn new(id: AgentId) -> Self {
        Self { id, step: 1 }
    }
}

impl Agent for WalkerAgent {
    fn id(&self) -> AgentId {
        self.id
    }

    fn decide(&mut self, tick: u64) -> AgentAction {
        // Simple oscillating walk along x-axis.
        if tick % 10 == 0 {
            self.step = -self.step;
        }
        AgentAction::Move { dx: self.step, dy: 0 }
    }
}

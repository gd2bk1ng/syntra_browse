// ================================================================================================
//   SYNTRA KERNEL — SIMULATION WORLD
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/simulation/world.rs
//   Module:      Simulation — World Model
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Discrete-time world model for agent-based simulations. Provides a simple grid-like
//                environment that can be extended into richer physics or symbolic worlds.
// ================================================================================================

use std::collections::HashMap;

use crate::simulation::agent::AgentId;

/// A simple 2D position in the simulated world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// World state: agents and their positions.
#[derive(Debug, Default)]
pub struct World {
    agents: HashMap<AgentId, Position>,
}

impl World {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn set_agent_position(&mut self, id: AgentId, pos: Position) {
        self.agents.insert(id, pos);
    }

    pub fn get_agent_position(&self, id: &AgentId) -> Option<Position> {
        self.agents.get(id).copied()
    }

    pub fn iter_agents(&self) -> impl Iterator<Item = (&AgentId, &Position)> {
        self.agents.iter()
    }
}

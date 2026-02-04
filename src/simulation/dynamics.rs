// ================================================================================================
//   SYNTRA KERNEL — SIMULATION DYNAMICS
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/simulation/dynamics.rs
//   Module:      Simulation — Dynamics Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Applies agent actions to the world state, advancing the simulation in discrete
//                ticks. This is the core loop for agent-based experiments.
// ================================================================================================

use crate::simulation::agent::{Agent, AgentAction, AgentId};
use crate::simulation::world::{Position, World};

/// Simulation engine: owns a world and a set of agents.
pub struct Simulation {
    pub world: World,
    pub agents: Vec<Box<dyn Agent>>,
    pub tick: u64,
}

impl Simulation {
    pub fn new(world: World, agents: Vec<Box<dyn Agent>>) -> Self {
        Self {
            world,
            agents,
            tick: 0,
        }
    }

    /// Advance the simulation by one tick.
    pub fn step(&mut self) {
        self.tick += 1;

        for agent in self.agents.iter_mut() {
            let id = agent.id();
            let action = agent.decide(self.tick);
            self.apply_action(id, action);
        }
    }

    fn apply_action(&mut self, id: AgentId, action: AgentAction) {
        let current = self.world.get_agent_position(&id).unwrap_or(Position { x: 0, y: 0 });

        let new_pos = match action {
            AgentAction::Stay => current,
            AgentAction::Move { dx, dy } => Position {
                x: current.x + dx,
                y: current.y + dy,
            },
        };

        self.world.set_agent_position(id, new_pos);
    }
}

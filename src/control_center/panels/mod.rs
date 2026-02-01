/* ================================================================================================
   SYNTRAOS — CONTROL CENTER PANELS
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/mod.rs
   Module:      SyntraOS Control Center — Panels Root
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Panels are the individual “views” inside the SyntraOS Control Center:
         • System
         • Cognition
         • Robotics
         • Smart Home
         • Security
         • Diagnostics
         • Evolution
         • Assistant
         • World Model
         • Predictive Engine
         • and more...

       Each panel is responsible for:
         • reading from ControlCenterState
         • providing a structured data model for UI layers
         • (future) rendering hints, layout metadata, animations

       Panels are UI-agnostic and do not draw pixels.
   ================================================================================================ */


pub mod cognition;

pub mod system;
pub mod environment;
pub mod robotics;
pub mod network;
pub mod smart_home;
pub mod security;
pub mod diagnostics;
pub mod evolution;
pub mod assistant;
pub mod world_model;
pub mod agents;
pub mod memory;
pub mod safety;
pub mod predictive;
pub mod simulation;
pub mod plugins;
pub mod developer;

pub mod ml;
pub mod continuity;

// Re-exports for ergonomic access
pub use system::SystemPanel;
pub use environment::EnvironmentPanel;
pub use robotics::RoboticsPanel;
pub use network::NetworkPanel;
pub use smart_home::SmartHomePanel;
pub use security::SecurityPanel;
pub use diagnostics::DiagnosticsPanel;
pub use evolution::EvolutionPanel;
pub use assistant::AssistantPanel;
pub use world_model::WorldModelPanel;
pub use agents::AgentsPanel;
pub use memory::MemoryPanel;
pub use safety::SafetyPanel;
pub use predictive::PredictivePanel;
pub use simulation::SimulationPanel;
pub use plugins::PluginsPanel;
pub use developer::DeveloperPanel;

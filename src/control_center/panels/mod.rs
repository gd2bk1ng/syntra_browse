/* ================================================================================================
   SYNTRAOS — CONTROL CENTER PANELS INDEX
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/mod.rs
   Module:      SyntraOS Control Center — Panels Index
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Index and re-exports for all Control Center panels.

       Panels:
         • system        — core OS status
         • cognition     — cognitive loop & context
         • robotics      — robot embodiment
         • network       — connectivity & throughput
         • smart_home    — home automation
         • security      — security posture
         • diagnostics   — health & integrity
         • evolution     — version lineage & growth
         • assistant     — conversational interface
         • world_model   — spatial & semantic world model
         • agents        — multi-agent runtime
         • memory        — cognitive memory
         • safety        — safety governance
         • predictive    — forecasting & trends
         • simulation    — sandbox & what-if engine
         • plugins       — extension ecosystem
         • developer     — dev-facing internals
   ================================================================================================ */

pub mod system;
pub mod cognition;
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

pub use system::SystemPanel;
pub use cognition::CognitionPanel;
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

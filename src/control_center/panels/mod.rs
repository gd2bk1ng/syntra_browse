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
       Central index and re-export hub for all Control Center panels.

       Panels provide *read-only*, structured, UI-agnostic access to subsystem state:
         • system        — core OS & hardware status
         • cognition     — cognitive loop, context & thoughtstream
         • robotics      — embodiment, locomotion, manipulation & sensors
         • network       — connectivity, throughput & interface health
         • smart_home    — automation, environment & device orchestration
         • security      — threat posture, sensors, cameras & zones
         • diagnostics   — system integrity, anomalies & health scoring
         • evolution     — capability lineage, scheduler & experiments
         • assistant     — conversational state, routing & reasoning trace
         • world_model   — spatial, semantic & temporal world representation
         • agents        — multi-agent runtime, roles, tasks & coordination
         • memory        — episodic, semantic, vector & cluster memory
         • safety        — governance, constraints, envelopes & risk scoring
         • predictive    — forecasting, trends, anomalies & projections
         • simulation    — physics, behavior, scenarios & sandbox state
         • plugins       — extension ecosystem, metadata, lifecycle & health
         • developer     — introspection, profiling, hot reload & dev tools

       Notes:
         • Panels are strictly read-only views.
         • All mutation flows through ControlCenterCommand.
         • This file is intentionally stable and future-proof.
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

// Re-exports for ergonomic access
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

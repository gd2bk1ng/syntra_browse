// ================================================================================================
//   SYNTRAOS — PROVIDER TRAITS
// ------------------------------------------------------------------------------------------------
//   File:        src/agi_core/providers.rs
//   Author: Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Trait-based provider interfaces for SyntraOS subsystems. Each provider is responsible for
//       updating a slice of the ControlCenterState. SyntraNode orchestrates them.
// ================================================================================================

#![allow(dead_code)]

use crate::control_center::ControlCenterState;

pub trait SystemProvider: Send {
    fn update_system(&mut self, state: &mut ControlCenterState);
}

pub trait CognitionProvider: Send {
    fn update_cognition(&mut self, state: &mut ControlCenterState);
}

pub trait RoboticsProvider: Send {
    fn update_robotics(&mut self, state: &mut ControlCenterState);
}

pub trait NetworkProvider: Send {
    fn update_network(&mut self, state: &mut ControlCenterState);
}

pub trait SmartHomeProvider: Send {
    fn update_smart_home(&mut self, state: &mut ControlCenterState);
}

pub trait SecurityProvider: Send {
    fn update_security(&mut self, state: &mut ControlCenterState);
}

pub trait WorldModelProvider: Send {
    fn update_world_model(&mut self, state: &mut ControlCenterState);
}

pub trait AgentsProvider: Send {
    fn update_agents(&mut self, state: &mut ControlCenterState);
}

pub trait MemoryProvider: Send {
    fn update_memory(&mut self, state: &mut ControlCenterState);
}

pub trait SafetyProvider: Send {
    fn update_safety(&mut self, state: &mut ControlCenterState);
}

pub trait PredictiveProvider: Send {
    fn update_predictive(&mut self, state: &mut ControlCenterState);
}

pub trait SimulationProvider: Send {
    fn update_simulation(&mut self, state: &mut ControlCenterState);
}

pub trait PluginsProvider: Send {
    fn update_plugins(&mut self, state: &mut ControlCenterState);
}

pub trait MlProvider: Send {
    fn update_ml(&mut self, state: &mut ControlCenterState);
}

pub trait ContinuityProvider: Send {
    fn update_continuity(&mut self, state: &mut ControlCenterState);
}

pub trait UiShellProvider: Send {
    fn update_ui_shell(&mut self, state: &mut ControlCenterState);
}

pub trait HudProvider: Send {
    fn update_hud(&mut self, state: &mut ControlCenterState);
}

/// Blanket trait for anything that wants a generic "tick" hook.
pub trait SyntraProvider: Send {
    fn tick(&mut self, _state: &mut ControlCenterState) {}
}

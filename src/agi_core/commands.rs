// ================================================================================================
//   SYNTRAOS — COMMAND BUS
// ------------------------------------------------------------------------------------------------
//   File:        src/agi_core/commands.rs
//   Author: Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Command model for SyntraOS. Assistant, UI, and automation issue commands; SyntraNode
//       executes them and updates ControlCenterState accordingly.
// ================================================================================================

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::control_center::{ControlCenterSection, SyntraSpace};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyntraCommand {
    // OS shell / navigation
    SwitchSpace(SyntraSpace),
    SwitchSection(ControlCenterSection),

    // Themes
    SetTheme(String),

    // Smart home
    ActivateScene(String),
    ToggleLight { room: String, device: String },
    LockDoor { door: String },
    UnlockDoor { door: String },

    // Security
    ArmSystem,
    DisarmSystem,

    // System
    RunDiagnostics,
    RescanEcosystem,

    // Session / continuity
    LockSession,
    // (future) UnlockSession,

    // Robotics
    RobotCommand(String),

    // Generic custom command hook
    Custom(String),
}

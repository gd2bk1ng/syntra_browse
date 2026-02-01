/* ================================================================================================
   SYNTRAOS — CONTROL CENTER COMMAND MODEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/commands.rs
   Module:      SyntraOS Control Center — Command Bus
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Unified command model for SyntraOS. Commands are issued by:
         • SyntraOS Shell (UI)
         • Syntra Browser (optional integration)
         • Assistant / Intent Bridge
         • Automation / Scenes
         • Robotics subsystems

       Commands mutate the ControlCenterState via SyntraNode::execute_command().
       This file defines the canonical command enum for all OS-level actions.
   ================================================================================================ */

use serde::{Deserialize, Serialize};

use super::state::{ControlCenterSection, SyntraSpace};

/// High-level OS commands that SyntraNode can execute.
///
/// These represent *intentional* state transitions or actions within SyntraOS.
/// They are safe, declarative, and UI-agnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyntraCommand {
    // --------------------------------------------------------------------------------------------
    // OS Shell / Navigation
    // --------------------------------------------------------------------------------------------
    /// Switch between SyntraOS spaces (Desktop, Robot HUD, Home, Console).
    SwitchSpace(SyntraSpace),

    /// Switch the active Control Center section/panel.
    SwitchSection(ControlCenterSection),

    // --------------------------------------------------------------------------------------------
    // Themes & Identity
    // --------------------------------------------------------------------------------------------
    /// Set the active theme by name.
    SetTheme(String),

    // --------------------------------------------------------------------------------------------
    // Smart Home
    // --------------------------------------------------------------------------------------------
    /// Activate a named smart home scene (e.g., "Night", "Studio", "Away").
    ActivateScene(String),

    /// Toggle a specific light in a room.
    ToggleLight {
        room: String,
        device: String,
    },

    /// Lock a specific door.
    LockDoor {
        door: String,
    },

    /// Unlock a specific door.
    UnlockDoor {
        door: String,
    },

    // --------------------------------------------------------------------------------------------
    // Security
    // --------------------------------------------------------------------------------------------
    /// Arm the security system.
    ArmSystem,

    /// Disarm the security system.
    DisarmSystem,

    // --------------------------------------------------------------------------------------------
    // System / Diagnostics
    // --------------------------------------------------------------------------------------------
    /// Trigger a system diagnostics run.
    RunDiagnostics,

    /// Re-scan the ecosystem (lobes, modules, health).
    RescanEcosystem,

    // --------------------------------------------------------------------------------------------
    // Session / Continuity
    // --------------------------------------------------------------------------------------------
    /// Lock the current session (UI lock screen).
    LockSession,

    // --------------------------------------------------------------------------------------------
    // Robotics
    // --------------------------------------------------------------------------------------------
    /// Generic robotics command (placeholder for HAL integration).
    RobotCommand(String),

    // --------------------------------------------------------------------------------------------
    // Custom / Extensibility
    // --------------------------------------------------------------------------------------------
    /// Arbitrary custom command payload.
    Custom(String),
}

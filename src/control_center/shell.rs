/* ================================================================================================
   SYNTRAOS — SHELL ENGINE (AXIOM THREE)
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/shell.rs
   Module:      SyntraOS Shell Engine
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       The SyntraOS Shell Engine is the high-level orchestrator that sits between:
         • SyntraNode (kernel runtime)
         • ControlCenterState (OS state model)
         • UI layers (desktop, browser, robot HUD, AR overlays)

       Responsibilities:
         • Provide a clean API for UI frontends to read OS state
         • Dispatch SyntraCommands to SyntraNode
         • Route SyntraEvents from SyntraNode to UI
         • Manage OS spaces (Desktop, Home, HUD, Console)
         • Manage Control Center panels
         • Future: animations, transitions, layout engine, HUD layers

       This module is intentionally UI-agnostic. It does not draw pixels or manage windows.
       It is the "OS Shell Brain" — the UI runtime that all frontends talk to.
   ================================================================================================ */

use std::sync::{Arc, Mutex};

use crate::agi_core::node::SyntraNode;
use crate::control_center::{
    commands::SyntraCommand,
    state::{ControlCenterSection, ControlCenterState, SyntraSpace},
};
use crate::agi_core::events::{EventBus, SyntraEvent};

/// Public handle for UI layers to interact with SyntraOS Shell.
///
/// This is what BrowserUI, RobotHUD, DesktopUI, or any other frontend receives.
/// It exposes:
///   - read-only access to ControlCenterState
///   - a command dispatch API
///   - an event subscription API
///
/// Internally, it wraps an Arc<Mutex<SyntraNode>>.
pub struct SyntraShell {
    node: Arc<Mutex<SyntraNode>>,
}

impl SyntraShell {
    /// Create a new shell engine from a SyntraNode handle.
    pub fn new(node: Arc<Mutex<SyntraNode>>) -> Self {
        Self { node }
    }

    // ============================================================================================
    //  STATE ACCESS
    // ============================================================================================

    /// Get a snapshot of the current ControlCenterState.
    ///
    /// UI layers should call this each frame or on-demand.
    pub fn state(&self) -> ControlCenterState {
        let node = self.node.lock().unwrap();
        node.control_center_state().clone()
    }

    /// Get the active OS space (Desktop, Home, HUD, Console).
    pub fn active_space(&self) -> SyntraSpace {
        let node = self.node.lock().unwrap();
        node.control_center_state().active_space
    }

    /// Get the active Control Center section (System, Smart Home, Security, etc.).
    pub fn active_section(&self) -> ControlCenterSection {
        let node = self.node.lock().unwrap();
        node.control_center_state().active_section
    }

    // ============================================================================================
    //  COMMAND DISPATCH
    // ============================================================================================

    /// Dispatch a SyntraCommand to the kernel runtime.
    ///
    /// UI layers call this to mutate OS state or trigger actions.
    pub fn dispatch(&self, cmd: SyntraCommand) {
        if let Ok(mut node) = self.node.lock() {
            node.execute_command(cmd);
        }
    }

    /// Convenience: switch OS space.
    pub fn switch_space(&self, space: SyntraSpace) {
        self.dispatch(SyntraCommand::SwitchSpace(space));
    }

    /// Convenience: switch Control Center section.
    pub fn switch_section(&self, section: ControlCenterSection) {
        self.dispatch(SyntraCommand::SwitchSection(section));
    }

    // ============================================================================================
    //  EVENT ROUTING
    // ============================================================================================

    /// Drain all pending SyntraEvents from the kernel.
    ///
    /// UI layers can poll this each frame to react to:
    ///   - notifications
    ///   - alerts
    ///   - state-changed events
    pub fn drain_events(&self) -> Vec<SyntraEvent> {
        if let Ok(mut node) = self.node.lock() {
            return node.events.drain();
        }
        Vec::new()
    }

    // ============================================================================================
    //  FUTURE: PANEL + SPACE RUNTIME
    // ============================================================================================

    /// Placeholder for future panel-specific logic.
    ///
    /// Example:
    ///   shell.panel("system").render(...)
    ///   shell.panel("smart_home").update(...)
    pub fn panel_runtime(&self) {
        // Reserved for future expansion.
    }

    /// Placeholder for future space-specific logic.
    ///
    /// Example:
    ///   shell.space(SyntraSpace::Desktop).layout(...)
    ///   shell.space(SyntraSpace::RobotHud).overlay(...)
    pub fn space_runtime(&self) {
        // Reserved for future expansion.
    }
}

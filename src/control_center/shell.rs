/* ================================================================================================
   SYNTRAOS — CONTROL CENTER SHELL ADAPTER
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/shell.rs
   Module:      SyntraOS Control Center — Shell Integration
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Thin adapter between the Control Center and the Syntra terminal shell.

       Responsibilities:
         • Map high-level shell commands to ControlCenterCommand
         • Provide read-only snapshots for UI rendering
         • Keep shell logic decoupled from internal state layout

       This module intentionally avoids any direct I/O.
       The terminal/CLI layer is responsible for printing and input handling.
   ================================================================================================ */

use crate::control_center::commands::ControlCenterCommand;
use crate::control_center::state::ControlCenterState;

#[derive(Debug)]
pub struct ControlCenterShell<'a> {
    pub state: &'a mut ControlCenterState,
}

impl<'a> ControlCenterShell<'a> {
    pub fn new(state: &'a mut ControlCenterState) -> Self {
        Self { state }
    }

    /// Applies a high-level command to the Control Center state.
    pub fn execute(&mut self, cmd: ControlCenterCommand) {
        cmd.apply(self.state);
    }

    /// Returns an immutable snapshot reference for rendering.
    pub fn snapshot(&self) -> &ControlCenterState {
        self.state
    }
}

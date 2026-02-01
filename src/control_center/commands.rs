/* ================================================================================================
   SYNTRAOS — CONTROL CENTER COMMANDS
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/commands.rs
   Module:      SyntraOS Control Center — Command API
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Mutation layer for the Control Center.

       Design:
         • Commands are explicit, auditable state transitions
         • Panels remain read-only views
         • Commands can be invoked from:
             – terminal (syntra shell)
             – browser dashboard
             – AGI core (agi_core::commands)
             – tests / benches

       Notes:
         • Keep commands coarse-grained and semantic (SetMode, ArmSecurity, etc.)
         • Avoid leaking low-level implementation details to callers
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

#[derive(Debug, Clone)]
pub enum ControlCenterCommand {
    SetSystemMode { mode: String },
    SetSafetyMode { mode: String, override_active: bool },
    ArmSecurity { armed: bool, mode: Option<String> },
    SetActiveScene { scene: String },
    SetAssistantMode { mode: String },
    PushDeveloperLog { entry: String },
}

impl ControlCenterCommand {
    pub fn apply(self, state: &mut ControlCenterState) {
        match self {
            ControlCenterCommand::SetSystemMode { mode } => {
                state.system.mode = Some(mode);
            }
            ControlCenterCommand::SetSafetyMode { mode, override_active } => {
                state.safety.mode = Some(mode);
                state.safety.override_active = override_active;
            }
            ControlCenterCommand::ArmSecurity { armed, mode } => {
                state.security.armed = armed;
                if mode.is_some() {
                    state.security.mode = mode;
                }
            }
            ControlCenterCommand::SetActiveScene { scene } => {
                state.smart_home.active_scene = Some(scene);
            }
            ControlCenterCommand::SetAssistantMode { mode } => {
                state.assistant.mode = Some(mode);
            }
            ControlCenterCommand::PushDeveloperLog { entry } => {
                state.developer.logs.push(entry.clone());
                state.developer.last_log = Some(entry);
            }
        }
    }
}

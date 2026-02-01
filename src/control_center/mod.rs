/* ================================================================================================
   SYNTRAOS — CONTROL CENTER ROOT
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/mod.rs
   Module:      SyntraOS Control Center — Root
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Entry point for the SyntraOS Control Center.

       Responsibilities:
         • Expose the ControlCenterState (single source of truth)
         • Expose all panels as typed views over state
         • Expose high-level commands for shell / terminal / UI
         • Provide a stable integration surface for the rest of syntra_kernel

       Design:
         • UI-agnostic (terminal, web, HUD, AR, etc.)
         • Read-only views via panels
         • Mutations via explicit commands
         • Safe to call from async and sync contexts
   ================================================================================================ */

pub mod state;
pub mod panels;
pub mod commands;
pub mod shell;

pub use state::ControlCenterState;
pub use panels::*;
pub use commands::ControlCenterCommand;
pub use shell::ControlCenterShell;

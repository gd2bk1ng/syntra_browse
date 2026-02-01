/* ================================================================================================
   SYNTRAOS — CONTROL CENTER CORE
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/mod.rs
   Module:      SyntraOS Control Center — Module Root
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Root module for the SyntraOS Control Center subsystem. Exposes:
         • Unified OS state model (state.rs)
         • SyntraOS command model (commands.rs)
         • Future: panels/, spaces/, shell runtime, event routing

       This module acts as the public API surface for the SyntraOS Shell and any UI layer
       (desktop, browser, robot HUD, AR overlay) that needs to read or manipulate OS state.
   ================================================================================================ */

pub mod state;
pub mod commands;

// Future expansion:
// pub mod panels;
// pub mod spaces;
// pub mod shell;

/* ================================================================================================
   SYNTRAOS — CONTROL CENTER PANELS
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/mod.rs
   Module:      SyntraOS Control Center — Panels Root
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Panels are the individual “views” inside the SyntraOS Control Center:
         • System
         • Cognition
         • Robotics
         • Smart Home
         • Security
         • Diagnostics
         • Evolution
         • Assistant
         • World Model
         • Predictive Engine
         • and more...

       Each panel is responsible for:
         • reading from ControlCenterState
         • providing a structured data model for UI layers
         • (future) rendering hints, layout metadata, animations

       Panels are UI-agnostic and do not draw pixels.
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
pub mod ml;
pub mod continuity;

/* ================================================================================================
   SYNTRAOS — ROBOTICS PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/robotics.rs
   Module:      SyntraOS Control Center — Robotics Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to robotics state:
         • Battery
         • Joint temperatures
         • Load
         • IMU orientation
         • Gait stability
         • Sensor health
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct RoboticsPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> RoboticsPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }
}

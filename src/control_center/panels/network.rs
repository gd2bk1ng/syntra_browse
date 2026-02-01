/* ================================================================================================
   SYNTRAOS — NETWORK PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/network.rs
   Module:      SyntraOS Control Center — Network Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to network state:
         • Connectivity status
         • Active interface
         • Throughput (up/down)
         • Latency
         • Packet loss
         • Network topology
         • Connected devices
         • Link health

       This panel is UI-agnostic. It prepares network data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct NetworkPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> NetworkPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    /// Returns whether the system is online.
    pub fn is_online(&self) -> bool {
        self.state.network.online
    }

    /// Returns the active network interface (e.g., "WiFi", "Ethernet").
    pub fn active_interface(&self) -> Option<String> {
        self.state.network.active_interface.clone()
    }

    /// Returns current upload throughput in Mbps.
    pub fn upload_mbps(&self) -> f32 {
        self.state.network.upload_mbps
    }

    /// Returns current download throughput in Mbps.
    pub fn download_mbps(&self) -> f32 {
        self.state.network.download_mbps
    }

    /// Returns current network latency in milliseconds.
    pub fn latency_ms(&self) -> f32 {
        self.state.network.latency_ms
    }

    /// Returns packet loss percentage.
    pub fn packet_loss_percent(&self) -> f32 {
        self.state.network.packet_loss_percent
    }

    /// Returns a list of connected devices (names or MACs).
    pub fn connected_devices(&self) -> &[String] {
        &self.state.network.connected_devices
    }

    /// Returns the network topology graph (if available).
    pub fn topology(&self) -> Option<String> {
        self.state.network.topology_map.clone()
    }
}

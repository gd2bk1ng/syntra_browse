// ================================================================================================
//   SYNTRAOS — EVENT BUS
// ------------------------------------------------------------------------------------------------
//   File:        src/agi_core/events.rs
//   Author: Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Lightweight event model + bus for SyntraOS. Used for notifications, alerts, and internal
//       signaling between subsystems and the UI.
// ================================================================================================

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::control_center::Notification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyntraEventKind {
    Notification(Notification),
    StateChanged(String), // e.g., "system", "smart_home", "robotics"
    Alert(String),        // high-level alert description
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntraEvent {
    pub kind: SyntraEventKind,
    pub source: String,
    pub timestamp: String,
}

impl SyntraEvent {
    pub fn notification(source: impl Into<String>, notification: Notification) -> Self {
        Self {
            kind: SyntraEventKind::Notification(notification),
            source: source.into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn state_changed(source: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            kind: SyntraEventKind::StateChanged(domain.into()),
            source: source.into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn alert(source: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            kind: SyntraEventKind::Alert(message.into()),
            source: source.into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Very simple in-memory event bus for now.
/// You can later replace this with a channel-based async bus.
#[derive(Default)]
pub struct EventBus {
    events: Vec<SyntraEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn publish(&mut self, event: SyntraEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<SyntraEvent> {
        std::mem::take(&mut self.events)
    }
}

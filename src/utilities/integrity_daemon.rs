// ================================================================================================
//   SYNTRA KERNEL — INTEGRITY DAEMON (PERIODIC WATCHDOG)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/integrity_daemon.rs
//   Module:      Utilities — Integrity Daemon (Watchdog)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       A periodic watchdog responsible for continuously monitoring Syntra’s integrity. This daemon
//       runs on a configurable interval and performs:
//
//         • Full tamper check via TamperMonitor
//         • Detection of unauthorized modifications
//         • Special attention to protected regions (AgiCore, Cortex)
//         • Automatic lockdown of self-modification capabilities when compromised
//         • Human override mechanism for restoring normal operation
//
//       This is Syntra’s “reflex arc” — a self-protective mechanism that prevents accidental or
//       malicious corruption while preserving her open-source nature.
//
//   Notes:
//       - Non-blocking; intended to run in a background thread or async task.
//       - Does NOT perform self-modification; only controls permission gates.
//       - MIT & Apache 2.0 dual-licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::utilities::{
    BaselineSnapshot,
    SemanticFsView,
    CodeIndex,
    DependencyGraph,
    TamperMonitor,
    IntegrityStatus,
};

/// Current operational mode of Syntra’s self-modification system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfModMode {
    /// Normal operation — self-mod allowed.
    Normal,

    /// Locked down — self-mod disabled until human override.
    LockedDown {
        reason: String,
        timestamp: Instant,
    },
}

/// Shared state for the daemon.
#[derive(Debug)]
pub struct IntegrityDaemonState {
    pub last_check: Option<Instant>,
    pub mode: SelfModMode,
}

/// Integrity daemon — periodic watchdog.
#[derive(Debug)]
pub struct IntegrityDaemon<B: crate::utilities::GitBackend> {
    tamper_monitor: TamperMonitor<B>,
    state: Arc<Mutex<IntegrityDaemonState>>,
    interval: Duration,
}

impl<B: crate::utilities::GitBackend> IntegrityDaemon<B> {
    pub fn new(
        tamper_monitor: TamperMonitor<B>,
        interval: Duration,
    ) -> Self {
        Self {
            tamper_monitor,
            state: Arc::new(Mutex::new(IntegrityDaemonState {
                last_check: None,
                mode: SelfModMode::Normal,
            })),
            interval,
        }
    }

    /// Returns whether Syntra is currently allowed to self-modify.
    pub fn self_mod_allowed(&self) -> bool {
        let state = self.state.lock().unwrap();
        matches!(state.mode, SelfModMode::Normal)
    }

    /// Human override — restores normal operation.
    pub fn human_override(&self, reason: &str) {
        let mut state = self.state.lock().unwrap();
        state.mode = SelfModMode::Normal;
        state.last_check = Some(Instant::now());
        println!("[IntegrityDaemon] Human override applied: {}", reason);
    }

    /// Run one integrity check cycle.
    pub fn run_once(
        &self,
        root: &Path,
        baseline: &BaselineSnapshot,
        semantic: &SemanticFsView,
        index: &CodeIndex,
        deps: &DependencyGraph,
    ) {
        let report = match self.tamper_monitor.check_integrity(
            root,
            baseline,
            semantic,
            index,
            deps,
            Some(50),
        ) {
            Ok(r) => r,
            Err(e) => {
                println!("[IntegrityDaemon] Error during integrity check: {}", e);
                return;
            }
        };

        let mut state = self.state.lock().unwrap();
        state.last_check = Some(Instant::now());

        match report.status {
            IntegrityStatus::Clean => {
                // If previously locked down, stay locked until human override.
                if matches!(state.mode, SelfModMode::Normal) {
                    println!("[IntegrityDaemon] Integrity clean.");
                }
            }

            IntegrityStatus::Suspicious => {
                println!("[IntegrityDaemon] Suspicious activity detected.");
            }

            IntegrityStatus::Compromised => {
                // Lock down self-modification.
                if !matches!(state.mode, SelfModMode::LockedDown { .. }) {
                    state.mode = SelfModMode::LockedDown {
                        reason: "Integrity compromised in protected region".into(),
                        timestamp: Instant::now(),
                    };
                    println!("[IntegrityDaemon] CRITICAL: Integrity compromised. Self-modification locked down.");
                }
            }
        }
    }

    /// Start the daemon loop (blocking).
    ///
    /// In real use, this would run in a background thread or async task.
    pub fn start_blocking(
        &self,
        root: &Path,
        baseline: BaselineSnapshot,
        semantic: SemanticFsView,
        index: CodeIndex,
        deps: DependencyGraph,
    ) {
        loop {
            self.run_once(root, &baseline, &semantic, &index, &deps);
            std::thread::sleep(self.interval);
        }
    }
}


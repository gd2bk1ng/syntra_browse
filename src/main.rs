/* ================================================================================================
   SYNTRA BROWSER — AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/main.rs
   Module:      Primary Executable Entrypoint
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Main executable entrypoint for the Syntra Browser. This file initializes logging,
                handles command-line arguments, invokes the Genesis bootstrap sequence, prepares
                the system for AGI-driven intent navigation and runtime expansion, and integrates
                continuous behavioral monitoring with session management.

   Execution Flow:
     1. Initialize structured logging + diagnostics.
     2. Parse CLI arguments (e.g., --demo).
     3. Invoke Genesis (system bootstrap).
     4. Initialize behavioral monitoring and session manager.
     5. Launch UI asynchronously.
     6. Monitor behavioral patterns continuously and lock session on anomalies.
     7. Listen for Ctrl+C for graceful shutdown.

   Notes:
     - Keeps the main binary minimal, observable, and extensible.
     - Behavioral monitoring runs silently in background.
     - Session locking triggers UI lock screen for secure re-authentication.
     - Compiler demos are reserved for a separate binary (syntra_compiler_demo).

   License: MIT
   Repository:  https://github.com/gd2bk1ng/syntra_browse
   ================================================================================================ */

use syntra_browse::genesis;
use std::env;
use syntra::browser::ui::BrowserUI;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

use syntra::browser::{
    behavioral_monitor::{BehavioralMonitor, TypingEvent},
    session_manager::SessionManager,
    input_handler,
};
use syntra_browse::agi_core::behavioral_profile::CreatorProfile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--demo" {
        println!("🔧 Syntra Reference Compiler Demo");
        println!("================================\n");
        println!("This flag is reserved for future in-process demos.");
        println!("For now, run the standalone demo binary:\n");
        println!("    cargo run --bin syntra_compiler_demo\n");
        return Ok(());
    }

    info!("🔮 Syntra Browser - Axiom Three");
    info!("🚀 Launching Genesis Sequence...");

    // Run genesis bootstrap (blocking wrapped in async)
    tokio::task::spawn_blocking(|| genesis::main()).await??;

    info!("🚀 Initializing Behavioral Monitor and Session Manager...");

    // Load creator behavioral profile (async if needed)
    let creator_profile = CreatorProfile::load().await.unwrap_or_default();

    // Initialize behavioral monitor and session manager
    let mut behavioral_monitor = BehavioralMonitor::new(creator_profile);
    let session_manager = SessionManager::new();

    // Channel for input events (simulate or connect real input source)
    let (input_tx, mut input_rx) = mpsc::channel::<TypingEvent>(100);

    // Spawn input event handler task
    let mut monitor_clone = behavioral_monitor.clone();
    let session_manager_clone = session_manager.clone();
    tokio::spawn(async move {
        while let Some(event) = input_rx.recv().await {
            input_handler::on_key_event(event, &mut monitor_clone, &session_manager_clone).await;
        }
    });

    info!("🚀 Launching UI...");

    // Run UI in blocking task if synchronous
    let ui_handle = tokio::task::spawn_blocking(|| BrowserUI::run());

    // Spawn periodic behavioral evaluation task
    let behavioral_monitor_ref = &behavioral_monitor;
    let session_manager_ref = &session_manager;
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            if !behavioral_monitor_ref.evaluate() && !session_manager_ref.is_locked() {
                info!("⚠️ Behavioral anomaly detected. Locking session...");
                session_manager_ref.lock().await;
                // Trigger UI lock screen or notification here
                // Example: ui::lock_screen::show_lock_screen().await;
            }
        }
    });

    // Graceful shutdown on Ctrl+C or UI task exit
    tokio::select! {
        res = ui_handle => {
            if let Err(e) = res? {
                error!("Browser crashed: {:?}", e);
                return Err(Box::new(e));
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down gracefully...");
            // Insert cleanup code here if needed
        }
    }

    info!("Shutdown complete.");
    Ok(())
}

// ================================================================================================
//   SYNTRA KERNEL — PRIMARY EXECUTABLE ENTRYPOINT
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/main.rs
//   Module:      Syntra Kernel — Main Entrypoint
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Launches the Syntra Kernel runtime, initializes diagnostics, loads behavioral
//                profiles, starts the UI subsystem, and orchestrates background cognitive and
//                security processes.
//
//   Execution Flow:
//     1. Initialize structured logging + diagnostics.
//     2. Parse CLI arguments.
//        • --demo       → compiler demo hint
//        • --terminal   → Syntra Terminal (CLI lobe)
//        • (default)    → Browser UI + behavioral monitor + SyntraOS runtime
//     3. Invoke Genesis bootstrap sequence.
//     4. Initialize SyntraNode (SyntraOS brain), behavioral monitor + session manager.
//     5. Launch UI (blocking or async depending on backend).
//     6. Spawn periodic behavioral evaluation loop.
//     7. Spawn periodic SyntraOS state refresh loop.
//     8. Listen for Ctrl+C for graceful shutdown.
// ================================================================================================

use std::env;
use std::sync::Arc;

use tokio::{
    signal,
    sync::{mpsc, Mutex},
    task,
    time::{self, Duration},
};

use tracing::{error, info};
use tracing_subscriber;

use syntra_kernel::agi_core::{
    behavioral_profile::CreatorProfile,
    node::SyntraNode,
    telemetry::TelemetryBus,
};
use syntra_kernel::browser::{
    behavioral_monitor::{BehavioralMonitor, TypingEvent},
    input_handler,
    session_manager::SessionManager,
    ui::BrowserUI,
};
use syntra_kernel::continuity::EpisodicMemory;
use syntra_kernel::diagnostics_ext::Profiler;
use syntra_kernel::genesis;
use syntra_kernel::security::Sandbox;
use syntra_kernel::terminal::run_cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --------------------------------------------------------------------------------------------
    // 1. Initialize Logging + Diagnostics
    // --------------------------------------------------------------------------------------------
    tracing_subscriber::fmt::init();
    info!("🔮 Syntra Kernel — Axiom Three");
    info!("Initializing diagnostics and runtime environment...");

    // Optional: attach advanced profiler
    let _profiler = Profiler::start_global();

    // --------------------------------------------------------------------------------------------
    // 2. Parse CLI Arguments (mode selection)
    // --------------------------------------------------------------------------------------------
    let args: Vec<String> = env::args().collect();

    // Dedicated demo flag (unchanged).
    if args.len() > 1 && args[1] == "--demo" {
        println!("🔧 Syntra Reference Compiler Demo");
        println!("================================\n");
        println!("This flag is reserved for future in-process demos.");
        println!("For now, run the standalone demo binary:\n");
        println!("    cargo run --bin syntra_compiler_demo\n");
        return Ok(());
    }

    // Terminal mode. This turns the main binary into the Syntra Terminal lobe.
    if args.len() > 1 && (args[1] == "--terminal" || args[1] == "terminal") {
        // run_cli() is synchronous and owns its own Cortex + Conduit.
        run_cli();
        return Ok(());
    }

    // Default: full browser/runtime stack.

    // --------------------------------------------------------------------------------------------
    // 3. Genesis Bootstrap
    // --------------------------------------------------------------------------------------------
    info!("🚀 Launching Genesis Sequence...");
    task::spawn_blocking(|| genesis::main()).await??;

    // --------------------------------------------------------------------------------------------
    // 4. Initialize SyntraOS Runtime (SyntraNode) + Behavioral Monitor + Session Manager
    // --------------------------------------------------------------------------------------------
    info!("🧠 Initializing SyntraOS runtime (SyntraNode)...");

    let telemetry = TelemetryBus::new("syntra_main".to_string());
    let root = std::env::current_dir()?;
    let syntra_node = SyntraNode::new(root, telemetry.clone());
    let syntra_node = Arc::new(Mutex::new(syntra_node));

    info!("🚀 Initializing Behavioral Monitor and Session Manager...");

    let creator_profile = CreatorProfile::load().await.unwrap_or_default();
    let mut behavioral_monitor = BehavioralMonitor::new(creator_profile);
    let session_manager = SessionManager::new();

    // Optional: continuity engine (episodic memory)
    let _episodic_memory = EpisodicMemory::new();

    // Optional: security sandbox (future integration)
    let _sandbox = Sandbox::new();

    // --------------------------------------------------------------------------------------------
    // 5. Input Event Channel + Handler Task
    // --------------------------------------------------------------------------------------------
    let (input_tx, mut input_rx) = mpsc::channel::<TypingEvent>(128);

    let mut monitor_clone = behavioral_monitor.clone();
    let session_manager_clone = session_manager.clone();

    tokio::spawn(async move {
        while let Some(event) = input_rx.recv().await {
            input_handler::on_key_event(event, &mut monitor_clone, &session_manager_clone).await;
        }
    });

    // `input_tx` is ready to be wired to real input sources later.
    let _ = input_tx;

    // --------------------------------------------------------------------------------------------
    // 6. Launch UI
    // --------------------------------------------------------------------------------------------
    info!("🚀 Launching UI...");
    // For now, BrowserUI::run() does not yet take a SyntraNode handle.
    // Later, you can extend it to accept Arc<Mutex<SyntraNode>> for live OS state binding.
    let ui_handle = task::spawn_blocking(|| BrowserUI::run());

    // --------------------------------------------------------------------------------------------
    // 7. Periodic Behavioral Evaluation Loop
    // --------------------------------------------------------------------------------------------
    let behavioral_monitor_ref = &behavioral_monitor;
    let session_manager_ref = &session_manager;

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;

            if !behavioral_monitor_ref.evaluate() && !session_manager_ref.is_locked() {
                info!("⚠️ Behavioral anomaly detected. Locking session...");
                session_manager_ref.lock().await;

                // Future: integrate cortex::ui::lock_screen
                // lock_screen::show().await;
            }
        }
    });

    // --------------------------------------------------------------------------------------------
    // 8. Periodic SyntraOS State Refresh Loop (SyntraNode::refresh_tick)
    // --------------------------------------------------------------------------------------------
    let syntra_node_clone = syntra_node.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(500));
        loop {
            interval.tick().await;
            if let Ok(mut node) = syntra_node_clone.lock().await {
                node.refresh_tick();
            }
        }
    });

    // --------------------------------------------------------------------------------------------
    // 9. Graceful Shutdown
    // --------------------------------------------------------------------------------------------
    tokio::select! {
        res = ui_handle => {
            if let Err(e) = res? {
                error!("UI subsystem crashed: {:?}", e);
                return Err(Box::new(e));
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received Ctrl+C — shutting down gracefully...");
        }
    }

    info!("Shutdown complete.");
    Ok(())
}

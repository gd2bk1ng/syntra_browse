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
//   Description:
//       Launches the Syntra Kernel runtime, initializes diagnostics, loads behavioral
//       profiles, starts the UI subsystem, and orchestrates background cognitive,
//       robotics, predictive, safety, and developer processes.
//
//       Now includes:
//         • ControlCenterState initialization
//         • ControlCenterShell binding
//         • Global shared state for UI + AGI runtime
//         • Periodic Control Center refresh loop
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

// NEW: Control Center
use syntra_kernel::control_center::{
    ControlCenterState,
    ControlCenterShell,
    ControlCenterCommand,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // --------------------------------------------------------------------------------------------
    // 1. Initialize Logging + Diagnostics
    // --------------------------------------------------------------------------------------------
    tracing_subscriber::fmt::init();
    info!("🔮 Syntra Kernel — Axiom Three");
    info!("Initializing diagnostics and runtime environment...");

    let _profiler = Profiler::start_global();

    // --------------------------------------------------------------------------------------------
    // 2. Parse CLI Arguments
    // --------------------------------------------------------------------------------------------
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--demo" {
        println!("🔧 Syntra Reference Compiler Demo");
        println!("================================\n");
        println!("Run: cargo run --bin syntra_compiler_demo\n");
        return Ok(());
    }

    if args.len() > 1 && (args[1] == "--terminal" || args[1] == "terminal") {
        run_cli();
        return Ok(());
    }

    // --------------------------------------------------------------------------------------------
    // 3. Genesis Bootstrap
    // --------------------------------------------------------------------------------------------
    info!("🚀 Launching Genesis Sequence...");
    task::spawn_blocking(|| genesis::main()).await??;

    // --------------------------------------------------------------------------------------------
    // 4. Initialize SyntraNode + Behavioral Monitor + Session Manager
    // --------------------------------------------------------------------------------------------
    info!("🧠 Initializing SyntraOS runtime (SyntraNode)...");

    let telemetry = TelemetryBus::new("syntra_main".to_string());
    let root = std::env::current_dir()?;
    let syntra_node = Arc::new(Mutex::new(SyntraNode::new(root, telemetry.clone())));

    info!("🚀 Initializing Behavioral Monitor and Session Manager...");

    let creator_profile = CreatorProfile::load().await.unwrap_or_default();
    let mut behavioral_monitor = BehavioralMonitor::new(creator_profile);
    let session_manager = SessionManager::new();

    let _episodic_memory = EpisodicMemory::new();
    let _sandbox = Sandbox::new();

    // --------------------------------------------------------------------------------------------
    // 5. Initialize Control Center (NEW)
    // --------------------------------------------------------------------------------------------
    info!("🧩 Initializing Control Center...");

    let control_center_state = Arc::new(Mutex::new(ControlCenterState::default()));

    // Optional: initial system mode
    {
        let mut cc = control_center_state.lock().await;
        ControlCenterCommand::SetSystemMode { mode: "booting".into() }.apply(&mut cc);
    }

    // --------------------------------------------------------------------------------------------
    // 6. Input Event Channel + Handler Task
    // --------------------------------------------------------------------------------------------
    let (input_tx, mut input_rx) = mpsc::channel::<TypingEvent>(128);

    let mut monitor_clone = behavioral_monitor.clone();
    let session_manager_clone = session_manager.clone();

    tokio::spawn(async move {
        while let Some(event) = input_rx.recv().await {
            input_handler::on_key_event(event, &mut monitor_clone, &session_manager_clone).await;
        }
    });

    let _ = input_tx;

    // --------------------------------------------------------------------------------------------
    // 7. Launch UI (Browser)
    // --------------------------------------------------------------------------------------------
    info!("🚀 Launching UI...");

    // Pass Control Center state into UI if needed
    let cc_for_ui = control_center_state.clone();

    let ui_handle = task::spawn_blocking(move || {
        BrowserUI::run_with_control_center(cc_for_ui)
    });

    // --------------------------------------------------------------------------------------------
    // 8. Periodic Behavioral Evaluation Loop
    // --------------------------------------------------------------------------------------------
    let behavioral_monitor_ref = behavioral_monitor.clone();
    let session_manager_ref = session_manager.clone();

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;

            if !behavioral_monitor_ref.evaluate() && !session_manager_ref.is_locked() {
                info!("⚠️ Behavioral anomaly detected. Locking session...");
                session_manager_ref.lock().await;
            }
        }
    });

    // --------------------------------------------------------------------------------------------
    // 9. Periodic SyntraNode Refresh Loop
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
    // 10. Periodic Control Center Refresh Loop (NEW)
    // --------------------------------------------------------------------------------------------
    let cc_clone = control_center_state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(750));
        loop {
            interval.tick().await;

            let mut cc = cc_clone.lock().await;

            // Example: update uptime or system metrics
            cc.system.uptime_seconds += 1;

            // Example: update developer heartbeat
            cc.developer.last_log = Some(format!("Heartbeat @ {:?}", chrono::Utc::now()));
        }
    });

    // --------------------------------------------------------------------------------------------
    // 11. Graceful Shutdown
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

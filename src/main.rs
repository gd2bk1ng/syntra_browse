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
                handles command-line arguments, invokes the Genesis bootstrap sequence, and prepares
                the system for AGI-driven intent navigation and future runtime expansion.

   Execution Flow:
     1. Initialize logging + diagnostics.
     2. Parse CLI arguments (e.g., --demo).
     3. Invoke Genesis (system bootstrap).
     4. Placeholder for future runtime components (renderer, AGI core, cortex).
     5. (Future) Enter main event loop.

   Notes:
     - Keeps the main binary minimal and observable.
     - Compiler demos are reserved for a separate binary (syntra_compiler_demo).
     - This version combines and updates previous axioms for clarity and extensibility.

   License: MIT
   Repository:  https://github.com/gd2bk1ng/syntra_browse
   ================================================================================================ */

async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// Entry point for the Syntra Browser executable.
///
/// Supported invocations:
///   syntra            — normal browser bootstrap
///   syntra --demo     — reserved for future in-process demos

use syntra_browse::genesis;
use std::env;
use syntra::browser::ui::BrowserUI;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

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

    // Run genesis bootstrap asynchronously if needed
    // If genesis::main() is sync, consider wrapping in spawn_blocking
    tokio::task::spawn_blocking(|| genesis::main()).await??;

    info!("🚀 Launching UI...");

    // Run UI in async context if BrowserUI supports async
    // If BrowserUI::run() is sync, run in blocking task
    let ui_handle = tokio::task::spawn_blocking(|| BrowserUI::run());

    // Listen for Ctrl+C signal for graceful shutdown
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

    // Placeholder for future runtime orchestration
    // syntra_browse::renderer::start().await?;
    // syntra_browse::agi_core::boot().await?;
    // syntra_browse::cortex::run().await?;

    info!("Shutdown complete.");
    Ok(())
}

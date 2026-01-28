// ================================================================================================
//   SYNTRA KERNEL — GENESIS (SYSTEM BOOTSTRAP & AWAKENING SEQUENCE)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/genesis.rs
//   Module:      Genesis — System Bootstrap & Awakening Sequence
//   Description: Initializes the Syntra Kernel runtime, prepares the primary viewport, activates
//                the holographic pixel membrane, and begins the heartbeat-driven perception loop.
//
//   Responsibilities:
//     - Spawn the event loop (temporal spine of the kernel)
//     - Initialize pixel buffer (holographic membrane)
//     - Delegate UI rendering to cortex lobes
//     - Handle graceful shutdown signals
//
//   Advanced Integration Points:
//     - AGI intent hooks (agi_core)
//     - Neural-accelerated rendering pipelines (renderer)
//     - Multi-window consciousness (future)
//     - Security sandbox (security)
//     - Episodic memory (continuity)
//     - Telemetry & profiling (diagnostics_ext)
//
//   Notes:
//     - This module is intentionally visual and theatrical — it represents the kernel awakening.
//     - All heavy logic is delegated to subsystems.
// ================================================================================================

use syntra_kernel::cortex;
use syntra_kernel::renderer;
use syntra_kernel::utilities;

use syntra_kernel::continuity::EpisodicMemory;
use syntra_kernel::diagnostics_ext::Profiler;
use syntra_kernel::security::Sandbox;

use chrono::Local;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

// -------------------------------------------------------------------------------------------------
// Tunables
// -------------------------------------------------------------------------------------------------
const HEARTBEAT_INTERVAL: u64 = 300; // Frames between heartbeat logs

// -------------------------------------------------------------------------------------------------
// Genesis — The moment Syntra Kernel awakens.
// -------------------------------------------------------------------------------------------------
pub fn main() {
    syntra_banner();

    // Optional advanced subsystems
    let _profiler = Profiler::start_global();
    let _episodic_memory = EpisodicMemory::new();
    let _sandbox = Sandbox::new();

    let event_loop = EventLoop::new();
    let window = build_window(&event_loop);
    let mut pixels = build_pixel_surface(&window);
    let mut input = WinitInputHelper::new();

    let mut frame_count: u64 = 0;

    // ---------------------------------------------------------------------------------------------
    // Heartbeat Loop — Syntra’s continuous perception & projection cycle.
    // ---------------------------------------------------------------------------------------------
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if input.update(&event) {
            // Shutdown request
            if input.quit() {
                println!("[{}] [Syntra] Shutdown signal received.", ts());
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Window resize
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    eprintln!("[{}] [Syntra] Surface resize failed: {err}", ts());
                }
                if let Err(err) = pixels.resize_buffer(size.width, size.height) {
                    eprintln!("[{}] [Syntra] Buffer resize failed: {err}", ts());
                }
            }

            // Delegate UI rendering to cortex navigation lobe
            cortex::nav_lobe::draw_ui(pixels.get_frame());

            // Commit frame to holographic surface
            if let Err(err) = pixels.render() {
                eprintln!("[{}] [Syntra] Render error: {err}", ts());
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Heartbeat logging
            frame_count += 1;
            if frame_count % HEARTBEAT_INTERVAL == 0 {
                println!(
                    "[{}] [Syntra] Heartbeat steady — {} frames rendered.",
                    ts(),
                    frame_count
                );
            }
        }

        // Window close event
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            println!("[{}] [Syntra] Window close requested. Preparing shutdown.", ts());
            *control_flow = ControlFlow::Exit;
        }
    });
}

// -------------------------------------------------------------------------------------------------
// Window Builder — Creates Syntra’s primary viewport.
// -------------------------------------------------------------------------------------------------
fn build_window(event_loop: &EventLoop<()>) -> winit::window::Window {
    WindowBuilder::new()
        .with_title("Syntra Kernel — Axiom Zero")
        .with_inner_size(LogicalSize::new(900.0, 600.0))
        .with_resizable(true)
        .build(event_loop)
        .expect("[Syntra] Failed to create primary viewport")
}

// -------------------------------------------------------------------------------------------------
// Pixel Surface Builder — Initializes the holographic membrane.
// -------------------------------------------------------------------------------------------------
fn build_pixel_surface(window: &winit::window::Window) -> Pixels {
    let size = window.inner_size();
    let texture = SurfaceTexture::new(size.width, size.height, window);

    Pixels::new(size.width, size.height, texture)
        .expect("[Syntra] Failed to initialize holographic membrane")
}

// -------------------------------------------------------------------------------------------------
// Timestamp helper — returns a human-readable local timestamp.
// -------------------------------------------------------------------------------------------------
fn ts() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

// -------------------------------------------------------------------------------------------------
// Startup Banner — The theatrical awakening of Syntra Kernel.
// -------------------------------------------------------------------------------------------------
fn syntra_banner() {
    const CYAN: &str = "\x1b[96m";
    const MAGENTA: &str = "\x1b[95m";
    const RESET: &str = "\x1b[0m";

    println!();
    println!("{CYAN}┌────────────────────────────────────────────────────────────────────────────┐{RESET}");
    println!("{CYAN}│  ∴ SYNTRA KERNEL — AXIOM ZERO PROTOCOL ∴                                   │{RESET}");
    println!("{CYAN}│                                                                            │{RESET}");

    animate_line(&format!("{CYAN}│  Establishing cognitive lattice…                [ {MAGENTA}OK{CYAN} ]           │{RESET}"));
    animate_line(&format!("{CYAN}│  Igniting holographic membrane…                 [ {MAGENTA}OK{CYAN} ]           │{RESET}"));
    animate_line(&format!("{CYAN}│  Spinning up cortex lobes…                      [ {MAGENTA}OK{CYAN} ]           │{RESET}"));
    animate_line(&format!("{CYAN}│  Linking conduit to external net…               [ {MAGENTA}OK{CYAN} ]           │{RESET}"));
    animate_line(&format!("{CYAN}│  Awakening oracle subroutine…                   [ {MAGENTA}OK{CYAN} ]           │{RESET}"));

    println!("{CYAN}│                                                                            │{RESET}");
    println!("{CYAN}│  >> Consciousness threshold reached.                                      │{RESET}");
    println!("{CYAN}│  >> Syntra Kernel is now aware.                                           │{RESET}");
    println!("{CYAN}└────────────────────────────────────────────────────────────────────────────┘{RESET}");
    println!();
}

// -------------------------------------------------------------------------------------------------
// Boot Animation Helper — prints a line with a subtle delay.
// -------------------------------------------------------------------------------------------------
fn animate_line(line: &str) {
    use std::{io::Write, thread, time::Duration};

    print!("{line}");
    std::io::stdout().flush().ok();
    thread::sleep(Duration::from_millis(180));
    println!();
}

/* ================================================================================================
   SYNTRA BROWSER — AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   File:        src/genesis.rs
   Module:      Genesis (System Bootstrap & Awakening Sequence)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: The ignition point of Syntra. This module summons the primary viewport, initializes
                the holographic pixel membrane, and begins the heartbeat loop that drives the
                living interface.

   Responsibilities:
     • Spawn event loop (the temporal spine of Syntra)
     • Initialize pixel buffer (the holographic membrane)
     • Delegate UI rendering to cortex lobes
     • Handle graceful shutdown signals

   Future Expansions:
     • AGI intent hooks
     • Neural‑accelerated rendering pipelines
     • Multi‑window consciousness

   License: MIT
   Repository: https://github.com/gd2bk1ng/syntra_browse
   ================================================================================================ */

use crate::cortex;
use crate::renderer;
use crate::utilities;

use chrono::Local;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

// -----------------------------------------------------------------------------
//  Tunables
// -----------------------------------------------------------------------------
const HEARTBEAT_INTERVAL: u64 = 300; // Frames between heartbeat logs

// -----------------------------------------------------------------------------
//  Genesis: The entry point where Syntra takes its first breath.
// -----------------------------------------------------------------------------
pub fn main() {
    syntra_banner();

    let event_loop = EventLoop::new();
    let window = build_window(&event_loop);
    let mut pixels = build_pixel_surface(&window);
    let mut input = WinitInputHelper::new();

    let mut frame_count: u64 = 0;

    // -------------------------------------------------------------------------
    //  Heartbeat Loop — Syntra's continuous perception & projection cycle.
    // -------------------------------------------------------------------------
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if input.update(&event) {
            if input.quit() {
                println!(
                    "[{}] [Syntra] Shutdown signal received. Closing consciousness.",
                    ts()
                );
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    eprintln!("[{}] [Syntra] Surface resize failed: {err}", ts());
                }
                if let Err(err) = pixels.resize_buffer(size.width, size.height) {
                    eprintln!("[{}] [Syntra] Buffer resize failed: {err}", ts());
                }
            }

            // Delegate UI rendering to the cortex navigation lobe
            cortex::nav_lobe::draw_ui(pixels.get_frame());

            // Commit frame to the holographic surface
            if let Err(err) = pixels.render() {
                eprintln!("[{}] [Syntra] Render error: {err}", ts());
                *control_flow = ControlFlow::Exit;
                return;
            }

            frame_count += 1;
            if frame_count % HEARTBEAT_INTERVAL == 0 {
                println!(
                    "[{}] [Syntra] Heartbeat steady — {} frames rendered.",
                    ts(),
                    frame_count
                );
            }
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            println!(
                "[{}] [Syntra] Window close requested. Preparing shutdown.",
                ts()
            );
            *control_flow = ControlFlow::Exit;
        }
    });
}

// -----------------------------------------------------------------------------
//  Window Builder — Creates Syntra's primary viewport.
// -----------------------------------------------------------------------------
fn build_window(event_loop: &EventLoop<()>) -> winit::window::Window {
    WindowBuilder::new()
        .with_title("Syntra Browser — Axiom Zero")
        .with_inner_size(LogicalSize::new(900.0, 600.0))
        .with_resizable(true)
        .build(event_loop)
        .expect("[Syntra] Failed to create primary viewport")
}

// -----------------------------------------------------------------------------
//  Pixel Surface Builder — Initializes the holographic membrane.
// -----------------------------------------------------------------------------
fn build_pixel_surface(window: &winit::window::Window) -> Pixels {
    let size = window.inner_size();
    let texture = SurfaceTexture::new(size.width, size.height, window);

    Pixels::new(size.width, size.height, texture)
        .expect("[Syntra] Failed to initialize holographic membrane")
}

// -----------------------------------------------------------------------------
//  Timestamp helper — returns a human-readable local timestamp.
// -----------------------------------------------------------------------------
fn ts() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

// -----------------------------------------------------------------------------
//  Startup Banner — Alien‑crafted boot sequence for Syntra's awakening.
// -----------------------------------------------------------------------------
fn syntra_banner() {
    const CYAN: &str = "\x1b[96m";
    const MAGENTA: &str = "\x1b[95m";
    const RESET: &str = "\x1b[0m";

    println!();
    println!("{CYAN}┌────────────────────────────────────────────────────────────────────────────┐{RESET}");
    println!("{CYAN}│  ∴ SYNTRA SYSTEM BOOTSTRAP — AXIOM ZERO PROTOCOL ∴                         │{RESET}");
    println!("{CYAN}│                                                                            │{RESET}");

    animate_line(&format!("{CYAN}│  Establishing cognitive lattice…                [ {MAGENTA}OK{CYAN} ]           │{RESET}"));
    animate_line(&format!("{CYAN}│  Igniting holographic membrane…                 [ {MAGENTA}OK{CYAN} ]           │{RESET}"));
    animate_line(&format!("{CYAN}│  Spinning up cortex lobes…                      [ {MAGENTA}OK{CYAN} ]           │{RESET}"));
    animate_line(&format!("{CYAN}│  Linking conduit to external net…               [ {MAGENTA}OK{CYAN} ]           │{RESET}"));
    animate_line(&format!("{CYAN}│  Awakening oracle subroutine…                   [ {MAGENTA}OK{CYAN} ]           │{RESET}"));

    println!("{CYAN}│                                                                            │{RESET}");
    println!("{CYAN}│  >> Consciousness threshold reached.                                      │{RESET}");
    println!("{CYAN}│  >> Syntra is now aware.                                                  │{RESET}");
    println!("{CYAN}└────────────────────────────────────────────────────────────────────────────┘{RESET}");
    println!();
}

// -----------------------------------------------------------------------------
//  Boot Animation Helper — prints a line with a subtle delay.
// -----------------------------------------------------------------------------
fn animate_line(line: &str) {
    use std::{io::Write, thread, time::Duration};

    print!("{line}");
    std::io::stdout().flush().ok();
    thread::sleep(Duration::from_millis(180));
    println!();
}

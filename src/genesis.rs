// -------------------------------------------------------------
//  Syntra Browser - Genesis Module (Entry Point)
//  Created by: Alexandr Roussinov (gd2bk1ng)
//  © 2026 Open Source - Licensed under MIT License
//
//  This is the main entry point of the Syntra Browser system.
//  It initializes the window, event loop, and pixel buffer rendering surface.
//  Delegates UI rendering and input handling to cortex modules.
//  Coordinates rendering pipeline and graceful shutdown.
//
//  Future expansions include integrating AGI assistant hooks and advanced rendering.
// -------------------------------------------------------------

mod cortex;
mod renderer;
mod utilities;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn main() {
    // Initialize the event loop for window events and rendering cycles
    let event_loop = EventLoop::new();

    // Build the main window with a futuristic title and default size
    let window = WindowBuilder::new()
        .with_title("Syntra Browser - The Future is Here")
        .with_inner_size(winit::dpi::LogicalSize::new(900, 600))
        .build(&event_loop)
        .expect("Failed to create window");

    // Create the pixel buffer surface for GPU-accelerated rendering
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture)
        .expect("Failed to create pixel buffer");

    // Input helper to handle keyboard, mouse, and window events efficiently
    let mut input = WinitInputHelper::new();

    // Run the event loop, handling inputs and rendering frames
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            // Request graceful exit on window close or ESC key
            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Handle dynamic window resizing by adjusting pixel buffer
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                pixels.resize_buffer(size.width, size.height);
            }

            // Delegate drawing to the cortex navigation lobe module
            cortex::nav_lobe::draw_ui(pixels.get_frame());

            // Render the pixel buffer to the window; exit on rendering errors
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Explicitly handle window close event to exit cleanly
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

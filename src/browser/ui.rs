/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/browser/ui.rs
   Module:      Browser UI Layer
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: High-level UI skeleton for the Syntra Browser. Defines the window, tab strip,
                address bar, and rendering surface using winit + pixels.

   Notes:
     - Axiom Three keeps UI minimal, fast, and GPU-accelerated.
     - This is the foundation for Syntra’s future AGI-native interface.
   ================================================================================================ */

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use pixels::{Pixels, SurfaceTexture};
use winit_input_helper::WinitInputHelper;

pub struct BrowserUI;

impl BrowserUI {
    pub fn run() -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title("Syntra Browser - Axiom Three")
            .with_inner_size(LogicalSize::new(1280.0, 800.0))
            .build(&event_loop)?;

        let mut input = WinitInputHelper::new();

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        let mut pixels = Pixels::new(size.width, size.height, surface_texture)?;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(size) => {
                        pixels.resize_surface(size.width, size.height);
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let frame = pixels.frame_mut();
                    Self::draw_browser_ui(frame);
                    pixels.render().unwrap();
                }
                _ => {}
            }

            if input.update(&event) {
                if input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                }
            }
        });
    }

    fn draw_browser_ui(frame: &mut [u8]) {
        // TODO: Replace with real GPU rendering
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[30, 30, 30, 255]); // dark gray background
        }
    }
}

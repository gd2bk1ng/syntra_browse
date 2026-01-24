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
   Description: Top-level UI runtime for Syntra Browser. Owns the window, event loop, pixel buffer,
                and delegates drawing to the tab strip, address bar, renderer, and AGI overlay.

   Notes:
     - Axiom Three keeps UI minimal, fast, and GPU-accelerated.
   ================================================================================================ */

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use pixels::{Pixels, SurfaceTexture};
use winit_input_helper::WinitInputHelper;

use crate::browser::tabs::TabStrip;
use crate::browser::address_bar::AddressBar;
use crate::browser::renderer::HtmlRenderer;
use crate::browser::agi_overlay::AgiOverlay;

pub struct BrowserUI {
    tabs: TabStrip,
    address_bar: AddressBar,
    renderer: HtmlRenderer,
    agi_overlay: AgiOverlay,
}

impl BrowserUI {
    pub fn new() -> Self {
        Self {
            tabs: TabStrip::new(),
            address_bar: AddressBar::new(),
            renderer: HtmlRenderer::new(),
            agi_overlay: AgiOverlay::new(),
        }
    }

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

        let mut ui = BrowserUI::new();

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
                    ui.draw(frame, size.width, size.height);
                    if let Err(e) = pixels.render() {
                        eprintln!("Pixels render error: {:?}", e);
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            }

            if input.update(&event) {
                if input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                }

                if let Some(text) = input.text() {
                    ui.address_bar.handle_text_input(text);
                }

                if input.key_pressed(winit::event::VirtualKeyCode::Return) {
                    if let Some(url) = ui.address_bar.current_url() {
                        ui.tabs.open_tab(url.clone());
                        ui.renderer.load_url(&url);
                    }
                }
            }
        });
    }

    fn draw(&mut self, frame: &mut [u8], width: u32, height: u32) {
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[18, 18, 18, 255]);
        }

        self.tabs.draw(frame, width, height);
        self.address_bar.draw(frame, width, height);
        self.renderer.draw(frame, width, height);
        self.agi_overlay.draw(frame, width, height);
    }
}

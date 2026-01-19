/* ================================================================================================
   SYNTRA BROWSER — AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   File:        src/renderer/mod.rs
   Module:      Renderer (Visual Output Pipeline)
   Author:      Alexandr Roussinov
   Description: Rendering abstraction layer for Syntra. Defines the core rendering trait and a
                baseline NullRenderer for early development and testing.

   Overview:
     • Renderer trait — Unified interface for all rendering backends.
     • NullRenderer   — Minimal placeholder renderer.

   Notes:
     Rendering should remain deterministic and decoupled from cognitive logic.
   ================================================================================================ */

#![allow(dead_code)]

/// Basic rendering interface for Syntra’s visual output pipeline.
pub trait Renderer {
    /// Called once per frame to draw the UI.
    fn render(&mut self);

    /// Called when the window is resized.
    fn resize(&mut self, width: u32, height: u32);
}

/// A placeholder renderer used during early development.
#[derive(Debug, Default)]
pub struct NullRenderer {
    pub width: u32,
    pub height: u32,
}

impl Renderer for NullRenderer {
    fn render(&mut self) {
        // Placeholder for future GPU or pixel‑based rendering.
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

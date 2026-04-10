/* ================================================================================================
   SYNTRA KERNEL :: BROWSER :: AXIOM THREE
   ------------------------------------------------------------------------------------------------
   File:        src/browser/renderer.rs
   Module:      Browser Content Renderer
   Author:      Alexandr Roussinov
   Description: Lightweight deterministic renderer for browser content area. Maintains a tiny
                render state so UI modules can inspect the active document and viewport details.
   ================================================================================================ */

/// Immutable snapshot of renderer state used by diagnostics/overlay modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderSnapshot {
    pub current_url: Option<String>,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub frame_counter: u64,
}

/// Minimal browser renderer used by `BrowserUI`.
#[derive(Debug, Default)]
pub struct HtmlRenderer {
    current_url: Option<String>,
    viewport_width: u32,
    viewport_height: u32,
    frame_counter: u64,
}

impl HtmlRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_url(&mut self, url: &str) {
        let normalized = url.trim();
        self.current_url = if normalized.is_empty() {
            None
        } else {
            Some(normalized.to_string())
        };
    }

    pub fn draw(&mut self, frame: &mut [u8], width: u32, height: u32) {
        self.viewport_width = width;
        self.viewport_height = height;
        self.frame_counter = self.frame_counter.saturating_add(1);

        let content_top = 70;
        let palette = if self.current_url.is_some() {
            [14, 28, 48, 255]
        } else {
            [26, 26, 32, 255]
        };

        for y in content_top..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&palette);
            }
        }

        // Draw a deterministic horizontal accent line to indicate content ready state.
        if self.current_url.is_some() && width > 40 && height > content_top + 24 {
            let y = content_top + 16;
            for x in 20..(width - 20) {
                let idx = ((y * width + x) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&[80, 120, 190, 255]);
            }
        }
    }

    pub fn snapshot(&self) -> RenderSnapshot {
        RenderSnapshot {
            current_url: self.current_url.clone(),
            viewport_width: self.viewport_width,
            viewport_height: self.viewport_height,
            frame_counter: self.frame_counter,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HtmlRenderer;

    #[test]
    fn loads_and_tracks_url() {
        let mut renderer = HtmlRenderer::new();
        renderer.load_url("  https://syntra.ai ");
        assert_eq!(
            renderer.snapshot().current_url.as_deref(),
            Some("https://syntra.ai")
        );
    }
}

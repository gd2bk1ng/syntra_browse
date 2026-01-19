/* ================================================================================================
   SYNTRA BROWSER — AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   File:        src/cortex/nav_lobe.rs
   Module:      Cortex — Navigation Lobe
   Author:      Alexandr Roussinov
   Description: Primitive UI/navigation lobe responsible for drawing the initial Syntra interface
                onto the pixel buffer. This is the first visual expression of Syntra’s mind.

   Overview:
     • draw_ui — Renders a simple, stylized frame into the pixel buffer.

   Notes:
     This is intentionally minimal and purely aesthetic. Future versions will evolve into a fully
     interactive, intent‑driven navigation cortex.
   ================================================================================================ */

#![allow(dead_code)]

/// Draws a simple UI frame into the given RGBA pixel buffer.
///
/// The buffer is expected to be a flat RGBA array (4 bytes per pixel).
pub fn draw_ui(frame: &mut [u8]) {
    let width = 900usize;
    let height = 600usize;

    // Safety: if the buffer size doesn't match, we just bail out gracefully.
    if frame.len() < width * height * 4 {
        return;
    }

    // Background: deep space gray
    for chunk in frame.chunks_exact_mut(4) {
        chunk[0] = 10;  // R
        chunk[1] = 12;  // G
        chunk[2] = 16;  // B
        chunk[3] = 255; // A
    }

    // Draw a subtle cyan border around the viewport.
    let border_thickness = 2usize;
    let cyan = [0u8, 220u8, 255u8, 255u8];

    for y in 0..height {
        for x in 0..width {
            let is_border = x < border_thickness
                || x >= width - border_thickness
                || y < border_thickness
                || y >= height - border_thickness;

            if is_border {
                let idx = (y * width + x) * 4;
                frame[idx..idx + 4].copy_from_slice(&cyan);
            }
        }
    }

    // Future: render text, glyphs, and dynamic UI elements here.
}

/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/browser/agi_overlay.rs
   Module:      AGI Overlay UI
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Minimal AGI overlay skeleton. Renders a translucent panel where AGI suggestions,
                intents, and explanations will appear.

   Notes:
     - Axiom Three keeps AGI behavior explicit and inspectable.
   ================================================================================================ */

pub struct AgiOverlay {
    pub visible: bool,
}

impl AgiOverlay {
    pub fn new() -> Self {
        Self { visible: true }
    }

    pub fn draw(&self, frame: &mut [u8], width: u32, height: u32) {
        if !self.visible {
            return;
        }

        let panel_width = width / 3;
        let panel_height = height / 3;
        let x0 = width - panel_width - 20;
        let y0 = 80;

        for y in y0..(y0 + panel_height) {
            for x in x0..(x0 + panel_width) {
                let idx = ((y * width + x) * 4) as usize;
                let existing = &frame[idx..idx + 4];
                let bg = [0u8, 0u8, 0u8, 180u8];
                let out = [
                    ((existing[0] as u16 + bg[0] as u16) / 2) as u8,
                    ((existing[1] as u16 + bg[1] as u16) / 2) as u8,
                    ((existing[2] as u16 + bg[2] as u16) / 2) as u8,
                    255,
                ];
                frame[idx..idx + 4].copy_from_slice(&out);
            }
        }
    }
}

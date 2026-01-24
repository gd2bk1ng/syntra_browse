/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/browser/address_bar.rs
   Module:      Address Bar
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Minimal address bar model and renderer. Tracks current URL text and exposes it to
                the UI runtime.

   Notes:
     - Axiom Three keeps input handling explicit and simple.
   ================================================================================================ */

#[derive(Debug, Default)]
pub struct AddressBar {
    buffer: String,
}

impl AddressBar {
    pub fn new() -> Self {
        Self {
            buffer: "https://example.com".into(),
        }
    }

    pub fn handle_text_input(&mut self, text: String) {
        self.buffer.push_str(&text);
    }

    pub fn current_url(&self) -> Option<String> {
        if self.buffer.trim().is_empty() {
            None
        } else {
            Some(self.buffer.trim().to_string())
        }
    }

    pub fn draw(&self, frame: &mut [u8], width: u32, _height: u32) {
        let top = 40;
        let bar_height = 30;
        for y in top..(top + bar_height) {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&[25, 25, 25, 255]);
            }
        }

        let text = self.buffer.as_bytes();
        let y = top + 10;
        for (i, _b) in text.iter().enumerate() {
            let x = 10 + (i as u32 * 6);
            if x + 4 >= width {
                break;
            }
            let idx = ((y * width + x) * 4) as usize;
            frame[idx..idx + 4].copy_from_slice(&[180, 180, 180, 255]);
        }
    }
}

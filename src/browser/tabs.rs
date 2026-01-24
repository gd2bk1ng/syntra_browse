/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/browser/tabs.rs
   Module:      Tab Strip
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Minimal tab strip model and renderer. Tracks open tabs and active tab index.

   Notes:
     - Axiom Three keeps the tab model simple and inspectable.
   ================================================================================================ */

#[derive(Debug, Clone)]
pub struct Tab {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Default)]
pub struct TabStrip {
    pub tabs: Vec<Tab>,
    pub active: usize,
}

impl TabStrip {
    pub fn new() -> Self {
        let mut s = Self::default();
        s.tabs.push(Tab {
            title: "New Tab".into(),
            url: "about:blank".into(),
        });
        s
    }

    pub fn open_tab(&mut self, url: String) {
        let title = url.clone();
        self.tabs.push(Tab { title, url });
        self.active = self.tabs.len() - 1;
    }

    pub fn draw(&self, frame: &mut [u8], width: u32, _height: u32) {
        let bar_height = 40;
        for y in 0..bar_height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&[40, 40, 40, 255]);
            }
        }

        if let Some(active) = self.tabs.get(self.active) {
            let text = format!(" {} ", active.title);
            let bytes = text.as_bytes();
            let y = 12;
            for (i, b) in bytes.iter().enumerate() {
                let x = 10 + (i as u32 * 6);
                if x + 4 >= width {
                    break;
                }
                let idx = ((y * width + x) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&[200, 200, 200, 255]);
                let _ = b;
            }
        }
    }
}

/* ================================================================================================
   SYNTRA BROWSER — AXIOM FIVE + SIX
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/browser/agi_overlay.rs
   Module:      AGI Overlay UI
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: AGI overlay panel. Renders a translucent UI region where Syntra’s current
                intent, classification, probability, and high-level plan summary are displayed.
                Integrated with Axiom Five (semantic reasoning) and Axiom Six (thought stream).

   Notes:
     - Overlay remains passive and non-autonomous (Axiom Three safety).
     - Displays only what the AGI Core produces.
     - Future versions may render multi-step plans or evolution proposals.
   ================================================================================================ */

use crate::agi_core::{
    Intent, IntentPlan, ProbReasoner, Reasoner, ThoughtStream, debug_plan,
};

pub struct AgiOverlay {
    pub visible: bool,
    reasoner: ProbReasoner,
    thoughts: ThoughtStream,
    last_plan: Option<IntentPlan>,
}

impl AgiOverlay {
    pub fn new() -> Self {
        Self {
            visible: true,
            reasoner: ProbReasoner::new(),
            thoughts: ThoughtStream::new(64),
            last_plan: None,
        }
    }

    /// Update the overlay with a new intent description.
    /// This is called by BrowserUI when navigation or user actions occur.
    pub fn update_intent(&mut self, description: &str) {
        let intent = Intent::new(description.to_string(), 1.0);

        // Axiom Five reasoning
        let plan = self.reasoner.process(intent);

        // Store in thought stream (Axiom Six)
        self.thoughts.push(plan.clone());

        // Keep last plan for overlay display
        self.last_plan = Some(plan);
    }

    /// Draw the translucent overlay panel.
    /// Currently renders only the background; text rendering will be added later.
    pub fn draw(&self, frame: &mut [u8], width: u32, height: u32) {
        if !self.visible {
            return;
        }

        let panel_width = width / 3;
        let panel_height = height / 3;
        let x0 = width - panel_width - 20;
        let y0 = 80;

        // Draw translucent background
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

        // --- TEXT RENDERING PLACEHOLDER ---------------------------------------------------------
        //
        // In the future, you will render:
        //   - plan.class
        //   - plan.probability
        //   - plan.intent
        //   - plan.plan (summary)
        //   - maybe multi-step plan
        //
        // For now, we simply keep the data available.
        //
        // Example (debug only):
        //
        // if let Some(plan) = &self.last_plan {
        //     println!("Overlay: {}", debug_plan(plan));
        // }
        //
        // -----------------------------------------------------------------------------------------
    }

    /// Expose the thought stream for terminal commands (/syntra:thoughts).
    pub fn thoughts(&self) -> &ThoughtStream {
        &self.thoughts
    }
}

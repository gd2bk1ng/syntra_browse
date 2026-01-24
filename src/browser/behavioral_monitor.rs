use crate::agi_core::behavioral_profile::CreatorProfile;

pub struct BehavioralMonitor {
    profile: CreatorProfile,
    recent_typing_data: Vec<TypingEvent>, // Define TypingEvent struct accordingly
}

impl BehavioralMonitor {
    pub fn new(profile: CreatorProfile) -> Self {
        Self {
            profile,
            recent_typing_data: Vec::new(),
        }
    }

    pub fn record_event(&mut self, event: TypingEvent) {
        self.recent_typing_data.push(event);
        if self.recent_typing_data.len() > MAX_BUFFER_SIZE {
            self.recent_typing_data.remove(0);
        }
    }

    pub fn evaluate(&self) -> bool {
        let live_pattern = extract_typing_pattern(&self.recent_typing_data);
        self.profile.matches(&live_pattern)
    }
}

// ================================================================================================
//   SYNTRA KERNEL — CONTINUITY STITCHING
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/continuity/stitching.rs
//   Module:      Continuity — Session & Episode Stitching
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Logic for stitching episodic traces, sessions, and memory fragments into coherent
//                long-term narratives. This is where Syntra reconstructs continuity across runs,
//                processes, and distributed nodes.
// ================================================================================================

use std::collections::VecDeque;

/// A single episodic event in Syntra’s continuity model.
#[derive(Debug, Clone)]
pub struct EpisodeEvent {
    pub timestamp_ms: u64,
    pub source: String,
    pub kind: String,
    pub payload: String,
}

/// A stitched narrative: ordered, filtered, and compressed view over events.
#[derive(Debug, Clone)]
pub struct Narrative {
    pub id: String,
    pub events: Vec<EpisodeEvent>,
    pub summary: Option<String>,
}

/// Strategy for stitching events into narratives.
pub trait StitchingStrategy: Send + Sync {
    /// Given a stream of events, produce one or more narratives.
    fn stitch(&self, events: &[EpisodeEvent]) -> Vec<Narrative>;
}

/// Simple time-window–based stitching strategy.
///
/// Groups events into narratives when gaps exceed `max_gap_ms` or when the source changes.
pub struct TimeWindowStitcher {
    pub max_gap_ms: u64,
}

impl TimeWindowStitcher {
    pub fn new(max_gap_ms: u64) -> Self {
        Self { max_gap_ms }
    }
}

impl StitchingStrategy for TimeWindowStitcher {
    fn stitch(&self, events: &[EpisodeEvent]) -> Vec<Narrative> {
        if events.is_empty() {
            return Vec::new();
        }

        let mut narratives = Vec::new();
        let mut current = VecDeque::new();

        let mut last_ts = events[0].timestamp_ms;
        let mut current_source = events[0].source.clone();

        for ev in events.iter().cloned() {
            let gap = ev.timestamp_ms.saturating_sub(last_ts);
            let source_changed = ev.source != current_source;

            if gap > self.max_gap_ms || source_changed {
                if !current.is_empty() {
                    narratives.push(build_narrative(&current));
                    current.clear();
                }
                current_source = ev.source.clone();
            }

            last_ts = ev.timestamp_ms;
            current.push_back(ev);
        }

        if !current.is_empty() {
            narratives.push(build_narrative(&current));
        }

        narratives
    }
}

fn build_narrative(events: &VecDeque<EpisodeEvent>) -> Narrative {
    let id = if let Some(first) = events.front() {
        format!("narrative-{}-{}", first.source, first.timestamp_ms)
    } else {
        "narrative-empty".to_string()
    };

    let summary = summarize(events);

    Narrative {
        id,
        events: events.iter().cloned().collect(),
        summary,
    }
}

/// Very lightweight summarization: just concatenates event kinds and truncates.
fn summarize(events: &VecDeque<EpisodeEvent>) -> Option<String> {
    if events.is_empty() {
        return None;
    }

    let mut parts: Vec<String> = events
        .iter()
        .map(|e| format!("{}:{}", e.source, e.kind))
        .collect();

    if parts.len() > 16 {
        parts.truncate(16);
        parts.push("…".to_string());
    }

    Some(parts.join(" | "))
}

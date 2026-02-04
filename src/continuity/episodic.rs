// ================================================================================================
//   SYNTRA KERNEL — CONTINUITY EPISODIC MEMORY
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/continuity/episodic.rs
//   Module:      Continuity — Episodic Memory Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: High‑level episodic memory subsystem for Syntra. This module records events,
//                organizes them into episodes, supports retrieval and filtering, and integrates
//                with persistence and stitching layers.
// ================================================================================================

use std::collections::{BTreeMap, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::continuity::persistence::{
    new_snapshot_id, PersistenceBackend, Snapshot, SnapshotId, SnapshotKind, SnapshotMeta,
};
use crate::continuity::stitching::{EpisodeEvent, Narrative, StitchingStrategy};

/// Unique identifier for an episode.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EpisodeId(pub String);

/// A single episodic memory entry.
#[derive(Debug, Clone)]
pub struct EpisodicRecord {
    pub timestamp_ms: u64,
    pub source: String,
    pub kind: String,
    pub payload: String,
    pub tags: Vec<String>,
}

/// A complete episode: a sequence of related events.
#[derive(Debug, Clone)]
pub struct Episode {
    pub id: EpisodeId,
    pub events: Vec<EpisodicRecord>,
    pub created_at: u64,
    pub summary: Option<String>,
}

/// Main episodic memory engine.
pub struct EpisodicMemory {
    /// Raw event log (append‑only).
    events: Vec<EpisodicRecord>,

    /// Episodes indexed by ID.
    episodes: BTreeMap<EpisodeId, Episode>,

    /// Optional stitching strategy.
    stitcher: Option<Box<dyn StitchingStrategy>>,

    /// Optional persistence backend.
    persistence: Option<Box<dyn PersistenceBackend>>,
}

impl EpisodicMemory {
    /// Create a new episodic memory engine.
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            episodes: BTreeMap::new(),
            stitcher: None,
            persistence: None,
        }
    }

    /// Attach a stitching strategy.
    pub fn with_stitcher(mut self, stitcher: Box<dyn StitchingStrategy>) -> Self {
        self.stitcher = Some(stitcher);
        self
    }

    /// Attach a persistence backend.
    pub fn with_persistence(mut self, backend: Box<dyn PersistenceBackend>) -> Self {
        self.persistence = Some(backend);
        self
    }

    /// Record a new episodic event.
    pub fn record(
        &mut self,
        source: impl Into<String>,
        kind: impl Into<String>,
        payload: impl Into<String>,
        tags: Vec<String>,
    ) {
        let ts = now_ms();
        let record = EpisodicRecord {
            timestamp_ms: ts,
            source: source.into(),
            kind: kind.into(),
            payload: payload.into(),
            tags,
        };
        self.events.push(record);
    }

    /// Build episodes using the configured stitching strategy.
    pub fn stitch(&mut self) -> Vec<EpisodeId> {
        let stitcher = match &self.stitcher {
            Some(s) => s,
            None => return Vec::new(),
        };

        // Convert to EpisodeEvent for stitching.
        let events: Vec<EpisodeEvent> = self
            .events
            .iter()
            .map(|e| EpisodeEvent {
                timestamp_ms: e.timestamp_ms,
                source: e.source.clone(),
                kind: e.kind.clone(),
                payload: e.payload.clone(),
            })
            .collect();

        let narratives: Vec<Narrative> = stitcher.stitch(&events);

        let mut ids = Vec::new();

        for n in narratives {
            let id = EpisodeId(n.id.clone());
            let created_at = now_ms();

            // Map narrative events back to episodic records.
            let mut evs = Vec::new();
            for ev in n.events {
                if let Some(orig) = self
                    .events
                    .iter()
                    .find(|r| r.timestamp_ms == ev.timestamp_ms && r.kind == ev.kind)
                {
                    evs.push(orig.clone());
                }
            }

            let episode = Episode {
                id: id.clone(),
                events: evs,
                created_at,
                summary: n.summary.clone(),
            };

            self.episodes.insert(id.clone(), episode);
            ids.push(id);
        }

        ids
    }

    /// Retrieve an episode by ID.
    pub fn get_episode(&self, id: &EpisodeId) -> Option<&Episode> {
        self.episodes.get(id)
    }

    /// Retrieve all episodes.
    pub fn all_episodes(&self) -> Vec<&Episode> {
        self.episodes.values().collect()
    }

    /// Filter episodes by tag.
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&Episode> {
        self.episodes
            .values()
            .filter(|ep| ep.events.iter().any(|e| e.tags.contains(&tag.to_string())))
            .collect()
    }

    /// Export an episode to persistence.
    pub fn persist_episode(&mut self, id: &EpisodeId) -> Option<SnapshotId> {
        let backend = self.persistence.as_mut()?;
        let episode = self.episodes.get(id)?;

        let meta = SnapshotMeta {
            id: new_snapshot_id("episode"),
            kind: SnapshotKind::Episodic,
            created_at: episode.created_at,
            tags: episode
                .events
                .iter()
                .flat_map(|e| e.tags.clone())
                .collect(),
        };

        let payload = serde_json::to_vec(episode).ok()?;

        Some(backend.store(Snapshot { meta, payload }))
    }

    /// Load an episode from persistence.
    pub fn load_episode(&mut self, id: &SnapshotId) -> Option<EpisodeId> {
        let backend = self.persistence.as_ref()?;
        let snapshot = backend.load(id)?;

        let episode: Episode = serde_json::from_slice(&snapshot.payload).ok()?;
        let eid = episode.id.clone();

        self.episodes.insert(eid.clone(), episode);
        Some(eid)
    }
}

/// Current timestamp in milliseconds.
fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ================================================================================================
//   SYNTRA KERNEL — CONTINUITY PERSISTENCE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/continuity/persistence.rs
//   Module:      Continuity — Persistence Layer
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Versioned, append-only persistence for episodic and semantic state. This module
//                provides an in-memory and file-backed abstraction for storing snapshots of
//                Syntra’s cognitive state, sessions, and long-term continuity artifacts.
// ================================================================================================

use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Unique identifier for a persisted snapshot.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SnapshotId(pub String);

/// High-level category of continuity data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SnapshotKind {
    /// Episodic memory (sessions, conversations, timelines).
    Episodic,
    /// Semantic memory (knowledge graphs, embeddings, indices).
    Semantic,
    /// System state (runtime, scheduler, actors, diagnostics).
    System,
}

/// Metadata describing a snapshot.
#[derive(Debug, Clone)]
pub struct SnapshotMeta {
    pub id: SnapshotId,
    pub kind: SnapshotKind,
    pub created_at: u64,
    pub tags: Vec<String>,
}

/// A stored snapshot: metadata + opaque payload bytes.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub meta: SnapshotMeta,
    pub payload: Vec<u8>,
}

/// Persistence backend abstraction.
pub trait PersistenceBackend: Send + Sync {
    /// Persist a snapshot and return its id.
    fn store(&mut self, snapshot: Snapshot) -> SnapshotId;

    /// Load a snapshot by id.
    fn load(&self, id: &SnapshotId) -> Option<Snapshot>;

    /// List all snapshots, optionally filtered by kind.
    fn list(&self, kind: Option<SnapshotKind>) -> Vec<SnapshotMeta>;

    /// Delete a snapshot by id.
    fn delete(&mut self, id: &SnapshotId) -> bool;
}

/// In-memory persistence backend (useful for tests and ephemeral runs).
#[derive(Default)]
pub struct InMemoryPersistence {
    snapshots: BTreeMap<SnapshotId, Snapshot>,
}

impl InMemoryPersistence {
    /// Create a new in-memory persistence backend.
    pub fn new() -> Self {
        Self {
            snapshots: BTreeMap::new(),
        }
    }
}

impl PersistenceBackend for InMemoryPersistence {
    fn store(&mut self, snapshot: Snapshot) -> SnapshotId {
        let id = snapshot.meta.id.clone();
        self.snapshots.insert(id.clone(), snapshot);
        id
    }

    fn load(&self, id: &SnapshotId) -> Option<Snapshot> {
        self.snapshots.get(id).cloned()
    }

    fn list(&self, kind: Option<SnapshotKind>) -> Vec<SnapshotMeta> {
        self.snapshots
            .values()
            .filter(|s| kind.map(|k| s.meta.kind == k).unwrap_or(true))
            .map(|s| s.meta.clone())
            .collect()
    }

    fn delete(&mut self, id: &SnapshotId) -> bool {
        self.snapshots.remove(id).is_some()
    }
}

/// Simple file-backed persistence backend.
///
/// Layout:
///   root_dir/
///     <snapshot_id>.bin
#[derive(Debug)]
pub struct FilePersistence {
    root: PathBuf,
}

impl FilePersistence {
    /// Create a new file-backed persistence backend at the given root directory.
    pub fn new<P: AsRef<Path>>(root: P) -> std::io::Result<Self> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    fn path_for(&self, id: &SnapshotId) -> PathBuf {
        self.root.join(format!("{}.bin", id.0))
    }
}

impl PersistenceBackend for FilePersistence {
    fn store(&mut self, snapshot: Snapshot) -> SnapshotId {
        let id = snapshot.meta.id.clone();
        let path = self.path_for(&id);

        // Very simple binary format: metadata as JSON + payload bytes.
        let meta_json = serde_json::to_vec(&SerializableMeta::from(&snapshot.meta))
            .expect("Failed to serialize snapshot metadata");

        let mut file = fs::File::create(path).expect("Failed to create snapshot file");
        let meta_len = meta_json.len() as u64;

        file.write_all(&meta_len.to_le_bytes())
            .expect("Failed to write meta length");
        file.write_all(&meta_json)
            .expect("Failed to write meta");
        file.write_all(&snapshot.payload)
            .expect("Failed to write payload");

        id
    }

    fn load(&self, id: &SnapshotId) -> Option<Snapshot> {
        let path = self.path_for(id);
        let mut file = fs::File::open(path).ok()?;

        let mut len_buf = [0u8; 8];
        file.read_exact(&mut len_buf).ok()?;
        let meta_len = u64::from_le_bytes(len_buf) as usize;

        let mut meta_buf = vec![0u8; meta_len];
        file.read_exact(&mut meta_buf).ok()?;
        let meta_ser: SerializableMeta = serde_json::from_slice(&meta_buf).ok()?;
        let meta = meta_ser.into_meta();

        let mut payload = Vec::new();
        file.read_to_end(&mut payload).ok()?;

        Some(Snapshot { meta, payload })
    }

    fn list(&self, _kind: Option<SnapshotKind>) -> Vec<SnapshotMeta> {
        // For now, we don’t index; caller can maintain an index using InMemoryPersistence
        // on top of FilePersistence if needed. This keeps the implementation simple.
        Vec::new()
    }

    fn delete(&mut self, id: &SnapshotId) -> bool {
        let path = self.path_for(id);
        fs::remove_file(path).is_ok()
    }
}

/// Serializable representation of `SnapshotMeta` for file persistence.
#[derive(serde::Serialize, serde::Deserialize)]
struct SerializableMeta {
    id: String,
    kind: String,
    created_at: u64,
    tags: Vec<String>,
}

impl From<&SnapshotMeta> for SerializableMeta {
    fn from(meta: &SnapshotMeta) -> Self {
        let kind = match meta.kind {
            SnapshotKind::Episodic => "episodic",
            SnapshotKind::Semantic => "semantic",
            SnapshotKind::System => "system",
        }
        .to_string();

        Self {
            id: meta.id.0.clone(),
            kind,
            created_at: meta.created_at,
            tags: meta.tags.clone(),
        }
    }
}

impl SerializableMeta {
    fn into_meta(self) -> SnapshotMeta {
        let kind = match self.kind.as_str() {
            "episodic" => SnapshotKind::Episodic,
            "semantic" => SnapshotKind::Semantic,
            "system" => SnapshotKind::System,
            _ => SnapshotKind::System,
        };

        SnapshotMeta {
            id: SnapshotId(self.id),
            kind,
            created_at: self.created_at,
            tags: self.tags,
        }
    }
}

/// Helper to create a new snapshot id with a timestamp prefix.
pub fn new_snapshot_id(prefix: &str) -> SnapshotId {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    SnapshotId(format!("{prefix}-{ts}"))
}

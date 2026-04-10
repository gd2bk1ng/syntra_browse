// ================================================================================================
//   SYNTRA KERNEL — CONTINUITY ENGINE
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/continuity/mod.rs
//   Module:      Syntra Kernel — Continuity Engine
//   Description: Long-term state persistence, episodic memory, and session stitching.
//                Provides temporal coherence across cognitive cycles.
//
//   Notes:
//     - Future integration: distributed continuity, cross-device state sync.
// ================================================================================================

pub mod episodic;
pub mod persistence;
pub mod stitching;

pub use episodic::EpisodicMemory;
pub use persistence::InMemoryPersistence as StateStore;
pub use stitching::TimeWindowSticher as SessionStitcher;

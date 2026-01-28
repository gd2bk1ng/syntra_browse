// ================================================================================================
//   SYNTRA KERNEL — KNOWLEDGE SUBSYSTEM
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/knowledge/mod.rs
//   Module:      Syntra Kernel — Knowledge Engine
//   Description: Semantic memory, embeddings, vector search, and knowledge graph operations.
//                Provides long-term memory and conceptual grounding for cognitive processes.
//
//   Notes:
//     - Designed for pluggable backends (in-memory, SQLite, vector DBs).
// ================================================================================================

pub mod embeddings;
pub mod graph;
pub mod memory;

pub use embeddings::EmbeddingSpace;
pub use graph::{KnowledgeGraph, Relation};
pub use memory::SemanticMemory;

// ================================================================================================
//   SYNTRA KERNEL — SECURITY SUBSYSTEM
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/security/mod.rs
//   Module:      Syntra Kernel — Security
//   Description: Capability-based security model, sandboxing, and intent-governed permissions.
//                Ensures safe execution of cognitive processes and external integrations.
//
//   Notes:
//     - Zero unsafe code.
//     - Designed for future WASM sandbox integration.
// ================================================================================================

pub mod capability;
pub mod sandbox;
pub mod policy;

pub use capability::Capability;
pub use policy::{Permission, Policy};
pub use sandbox::Sandbox;

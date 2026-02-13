// ================================================================================================
//   SYNTRA KERNEL — RUNTIME CORE (ACTORS, SCHEDULER, TENSORS)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/runtime/mod.rs
//   Module:      Runtime Core
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Core runtime primitives for the Syntra Kernel: actor system, scheduler, and tensor
//                engine. Provides the execution backbone for cognitive processes, pipelines, and
//                higher-level orchestration.
//
//   Notes:
//     - Axiom Three defines the shape of the runtime without overcommitting to a specific backend.
//     - Designed to integrate with diagnostics_ext, continuity, and distributed subsystems.
// ================================================================================================

pub mod actor;
pub mod actor_executor;
pub mod scheduler;
pub mod tensor;
pub mod cognitive_runtime;


// -------------------------------------------------------------------------------------------------
// Public Re-exports
// -------------------------------------------------------------------------------------------------

pub use actor::{Actor, ActorId, ActorMessage};
pub use actor_executor::ActorExecutor;
pub use scheduler::{RuntimeScheduler, SchedulerConfig};
pub use tensor::{Tensor, TensorShape};


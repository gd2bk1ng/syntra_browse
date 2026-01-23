/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/runtime/mod.rs
   Module:      Runtime Core (Actors, Scheduler, Tensors)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Core runtime primitives for Syntra Browser: actor system, scheduler, and tensor
                engine skeletons.

   Notes:
     - Axiom Three defines the shape of the runtime without overcommitting to an implementation.
   ================================================================================================ */

pub mod actor;
pub mod tensor;
pub mod scheduler;

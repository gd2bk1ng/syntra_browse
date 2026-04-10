// ================================================================================================
//   SYNTRA KERNEL :: PREDICTIVE ENGINE
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/predictive/mod.rs
//   Module:      Syntra Kernel :: Predictive Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Forecasting engine, temporal reasoning, and pattern modeling.
//                Enables anticipatory cognition and future-state estimation.
//
//   Notes:
//     - Supports pluggable models (statistical, ML, hybrid).
// ================================================================================================

pub mod temporal;
pub mod forecasting;
pub mod patterns;

pub use forecasting::Forecaster;
pub use patterns::PatternModel;
pub use temporal::TemporalReasoner;

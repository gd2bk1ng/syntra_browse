// ================================================================================================
//   SYNTRA KERNEL — ADVANCED DIAGNOSTICS
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s'
//
//   File:        src/diagnostics_ext/mod.rs
//   Module:      Syntra Kernel — Advanced Diagnostics
//   Description: High-resolution profiling, telemetry, and performance analytics.
//                Complements the basic utilities diagnostics.
//
//   Notes:
//     - Designed for integration with flamegraphs and tracing frameworks.
// ================================================================================================

pub mod profiler;
pub mod telemetry;
pub mod metrics;

pub use metrics::MetricCollector;
pub use profiler::Profiler;
pub use telemetry::TelemetryStream;

// ================================================================================================
//   SYNTRA KERNEL — ADVANCED DIAGNOSTICS
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/diagnostics_ext/mod.rs
//   Module:      Syntra Kernel :: Advanced Diagnostics
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: High-resolution profiling, telemetry, and performance analytics.
//                Complements the basic utilities diagnostics.
//
//   Notes:
//     - Designed for integration with flamegraphs and tracing frameworks.
// ================================================================================================

pub mod profiler;
pub mod telemetry;
pub mod metrics;

pub use metrics::MetricsRegistry as MetricCollector;
pub use profiler::Profiler;
pub use telemetry::TelemetryBus as TelemetryStream;


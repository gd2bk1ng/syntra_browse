// ================================================================================================
//   SYNTRA KERNEL — DIAGNOSTICS PROFILER
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/diagnostics_ext/profiler.rs
//   Module:      Diagnostics — Lightweight Profiler
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Scoped timing and profiling utilities for measuring hot paths in the Syntra
//                kernel. Designed to be zero-cost when disabled and low-overhead when enabled.
// ================================================================================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A single profiling sample.
#[derive(Debug, Clone)]
pub struct ProfileSample {
    pub label: String,
    pub duration: Duration,
}

/// Aggregated statistics for a label.
#[derive(Debug, Default, Clone)]
pub struct ProfileStats {
    pub count: u64,
    pub total: Duration,
    pub max: Duration,
}

impl ProfileStats {
    pub fn record(&mut self, d: Duration) {
        self.count += 1;
        self.total += d;
        if d > self.max {
            self.max = d;
        }
    }

    pub fn avg(&self) -> Option<Duration> {
        if self.count == 0 {
            None
        } else {
            Some(self.total / self.count)
        }
    }
}

/// Global profiler registry.
#[derive(Default, Clone)]
pub struct Profiler {
    inner: Arc<Mutex<HashMap<String, ProfileStats>>>,
}

impl Profiler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a scoped timer for the given label.
    pub fn scope<'a>(&'a self, label: impl Into<String>) -> ProfileScope<'a> {
        ProfileScope {
            profiler: self,
            label: label.into(),
            start: Instant::now(),
        }
    }

    /// Record a completed sample.
    pub fn record(&self, label: &str, duration: Duration) {
        let mut guard = self.inner.lock().expect("profiler poisoned");
        let stats = guard.entry(label.to_string()).or_default();
        stats.record(duration);
    }

    /// Snapshot current stats.
    pub fn snapshot(&self) -> HashMap<String, ProfileStats> {
        self.inner.lock().expect("profiler poisoned").clone()
    }
}

/// RAII scope for profiling a block of code.
pub struct ProfileScope<'a> {
    profiler: &'a Profiler,
    label: String,
    start: Instant,
}

impl<'a> Drop for ProfileScope<'a> {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        self.profiler.record(&self.label, duration);
    }
}

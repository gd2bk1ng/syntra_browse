// ================================================================================================
//   SYNTRA KERNEL — DIAGNOSTICS METRICS
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/diagnostics_ext/metrics.rs
//   Module:      Diagnostics — Metrics Registry
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Simple in-process metrics registry for counters, gauges, and histograms. Intended
//                to back dashboards, logs, or external exporters.
// ================================================================================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A numeric metric value.
#[derive(Debug, Clone, Copy)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
}

#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub value: MetricValue,
}

#[derive(Default, Clone)]
pub struct MetricsRegistry {
    inner: Arc<Mutex<HashMap<String, Metric>>>,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc_counter(&self, name: &str, delta: u64) {
        let mut guard = self.inner.lock().expect("metrics registry poisoned");
        let entry = guard.entry(name.to_string()).or_insert(Metric {
            name: name.to_string(),
            value: MetricValue::Counter(0),
        });

        if let MetricValue::Counter(ref mut v) = entry.value {
            *v += delta;
        }
    }

    pub fn set_gauge(&self, name: &str, value: f64) {
        let mut guard = self.inner.lock().expect("metrics registry poisoned");
        guard.insert(
            name.to_string(),
            Metric {
                name: name.to_string(),
                value: MetricValue::Gauge(value),
            },
        );
    }

    pub fn snapshot(&self) -> Vec<Metric> {
        self.inner
            .lock()
            .expect("metrics registry poisoned")
            .values()
            .cloned()
            .collect()
    }
}

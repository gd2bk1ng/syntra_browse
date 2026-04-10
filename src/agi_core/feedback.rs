// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (FEEDBACK LOOP & SELF-IMPROVEMENT ENGINE)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/feedback.rs
//   Module:      Feedback Loop & Self-Improvement Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Syntra Kernel’s multi-source feedback architecture for self-improvement and continuous
//       evolution. Unifies user feedback, system telemetry, and simulation results into a modular,
//       extensible pipeline for adaptive learning.
//
//   Architectural Role:
//       • Axiom Three — Cognitive Awareness (closing the loop on reasoning quality).
//       • Axiom Four  — Routing refinement (improving perception/cognition/action decisions).
//       • Axiom Six   — Evolution (data-driven self-modification).
//
//   Overview:
//       • Feedback                — UserFeedback, SystemTelemetry, SimulationResult
//       • FeedbackStore           — In-memory, versioned feedback repository
//       • FeedbackProcessor       — Aggregates, validates, dispatches feedback
//       • FeedbackUpdateStrategy  — Trait for rule/model update plugins
//       • RuleUpdater             — Deterministic rule adjustment engine
//       • ModelTrainer            — Stub for ML retraining hooks
//       • AsyncFeedbackIngestor   — Tokio-based async ingestion API
//
//   Notes:
//       - No direct I/O; integration happens at the edges (UI, API, logs).
//       - Designed to be safe to call from any lobe.
//       - Future: federated learning, neural-symbolic fusion, multi-modal feedback.
// ================================================================================================

#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::agi_core::telemetry::{TelemetryEvent, TelemetryLevel};
use crate::agi_core::safety::SafetyVerdict;

// ================================================================================================
// Core Feedback Types
// ================================================================================================

/// Core feedback types for Syntra Kernel’s self-improvement cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Feedback {
    /// Explicit user feedback on classification or plan execution.
    User(UserFeedback),
    /// Automated system telemetry capturing runtime behavior and anomalies.
    Telemetry(SystemTelemetry),
    /// Synthetic feedback from simulation runs for validation and stress-testing.
    Simulation(SimulationResult),
}

/// Explicit user feedback on intent classification or plan execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    pub feedback_id: Uuid,
    pub intent_label: String,
    /// 1–5 rating (e.g., stars or satisfaction score).
    pub user_rating: u8,
    pub comments: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Automated system telemetry capturing runtime behavior and anomalies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTelemetry {
    pub telemetry_id: Uuid,
    pub intent_label: String,
    pub error_code: Option<String>,
    /// Normalized anomaly score in [0.0, 1.0].
    pub anomaly_score: f32,
    pub details: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Synthetic feedback from simulation runs for validation and stress-testing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub simulation_id: Uuid,
    pub test_case: String,
    pub expected_intent: String,
    pub actual_intent: String,
    pub success: bool,
    pub notes: Option<String>,
    pub timestamp: DateTime<Utc>,
}

// ================================================================================================
// Traits for Extensibility
// ================================================================================================

/// Trait for feedback sources to implement for plug-and-play extensibility.
pub trait FeedbackSource: Send + Sync {
    fn collect_feedback(&self) -> Vec<Feedback>;
}

/// Trait for update strategies that consume aggregated feedback.
///
/// Implementations can:
///   • Adjust deterministic rules
///   • Trigger ML retraining
///   • Emit alerts or telemetry
pub trait FeedbackUpdateStrategy: Send + Sync {
    fn update(&self, feedback_batch: &[Feedback]);
}

// ================================================================================================
// Feedback Store (In-Memory, Versioned)
// ================================================================================================

/// In-memory store for feedback with versioned, timestamped batches.
///
/// This is intentionally simple and side-effect free; persistence is
/// expected to be handled by higher-level services if needed.
pub struct FeedbackStore {
    store: Mutex<HashMap<Uuid, Feedback>>,
    history: Mutex<VecDeque<(DateTime<Utc>, Vec<Feedback>)>>, // Timestamped batches
    max_history_len: usize,
}

impl FeedbackStore {
    /// Create a new FeedbackStore with a maximum history length.
    pub fn new(max_history_len: usize) -> Self {
        FeedbackStore {
            store: Mutex::new(HashMap::new()),
            history: Mutex::new(VecDeque::with_capacity(max_history_len)),
            max_history_len,
        }
    }

    /// Add a single feedback item to the store and history.
    pub fn add_feedback(&self, feedback: Feedback) {
        let mut store_guard = self.store.lock().expect("FeedbackStore lock poisoned");
        let id = match &feedback {
            Feedback::User(fb) => fb.feedback_id,
            Feedback::Telemetry(fb) => fb.telemetry_id,
            Feedback::Simulation(fb) => fb.simulation_id,
        };
        store_guard.insert(id, feedback.clone());
        drop(store_guard);

        let mut history_guard = self.history.lock().expect("FeedbackStore history lock poisoned");
        let now = Utc::now();

        if let Some(last_batch) = history_guard.back_mut() {
            if (now - last_batch.0).num_seconds() < 60 {
                last_batch.1.push(feedback);
                return;
            }
        }

        history_guard.push_back((now, vec![feedback]));
        if history_guard.len() > self.max_history_len {
            history_guard.pop_front();
        }
    }

    /// Retrieve all feedback items added after a given timestamp.
    pub fn get_feedback_batch(&self, since: DateTime<Utc>) -> Vec<Feedback> {
        let history_guard = self.history.lock().expect("FeedbackStore history lock poisoned");
        history_guard
            .iter()
            .filter(|(ts, _)| *ts > since)
            .flat_map(|(_, batch)| batch.clone())
            .collect()
    }
}

// ================================================================================================
// Feedback Processor
// ================================================================================================

/// Core processor that ingests feedback, validates it, and dispatches updates.
///
/// This is the central hub of the feedback loop. It:
///   • Validates incoming feedback
///   • Stores it in FeedbackStore
///   • Dispatches it to registered update strategies
pub struct FeedbackProcessor {
    store: Arc<FeedbackStore>,
    update_strategies: Vec<Arc<dyn FeedbackUpdateStrategy>>,
}

impl FeedbackProcessor {
    /// Create a new FeedbackProcessor bound to a FeedbackStore.
    pub fn new(store: Arc<FeedbackStore>) -> Self {
        FeedbackProcessor {
            store,
            update_strategies: Vec::new(),
        }
    }

    /// Register a new update strategy.
    pub fn register_strategy(&mut self, strategy: Arc<dyn FeedbackUpdateStrategy>) {
        self.update_strategies.push(strategy);
    }

    /// Ingest a batch of feedback, store it, validate it, and trigger updates.
    pub fn process_feedback_batch(&self, feedback_batch: Vec<Feedback>) {
        info!("Processing feedback batch of size {}", feedback_batch.len());

        let validated_feedback: Vec<Feedback> = feedback_batch
            .into_iter()
            .filter(|fb| self.validate_feedback(fb))
            .collect();

        for fb in &validated_feedback {
            self.store.add_feedback(fb.clone());
        }

        for strategy in &self.update_strategies {
            strategy.update(&validated_feedback);
        }

        info!("Feedback batch processed and dispatched to update strategies.");
    }

    /// Basic validation logic to filter out malformed or suspicious feedback.
    fn validate_feedback(&self, feedback: &Feedback) -> bool {
        match feedback {
            Feedback::User(fb) => (1..=5).contains(&fb.user_rating),
            Feedback::Telemetry(fb) => (0.0..=1.0).contains(&fb.anomaly_score),
            Feedback::Simulation(_) => true, // Assume simulation results are valid
        }
    }
}

// ================================================================================================
// Rule Updater (Deterministic Rules)
// ================================================================================================

/// Deterministic rule updater that adjusts classification rules based on feedback.
///
/// This is the symbolic side of Axiom Six: using structured feedback to
/// refine thresholds, keyword weights, and routing decisions.
pub struct RuleUpdater {
    // Placeholder for rule data structures, e.g., keyword weights, thresholds.
}

impl RuleUpdater {
    pub fn new() -> Self {
        RuleUpdater {
            // Future: initialize rule sets or load from config.
        }
    }

    /// Core update method applying feedback insights to rules.
    pub fn apply_feedback(&self, feedback_batch: &[Feedback]) {
        for fb in feedback_batch {
            match fb {
                Feedback::User(user_fb) => {
                    if user_fb.user_rating < 3 {
                        warn!(
                            "Low user rating for intent '{}': {} stars. Comments: {:?}",
                            user_fb.intent_label, user_fb.user_rating, user_fb.comments
                        );
                        // TODO: Implement rule adjustment logic here.
                    }
                }
                Feedback::Telemetry(telemetry) => {
                    if telemetry.anomaly_score > 0.7 {
                        warn!(
                            "High anomaly score for intent '{}': {}. Details: {:?}",
                            telemetry.intent_label, telemetry.anomaly_score, telemetry.details
                        );
                        // TODO: Implement rule tuning or alerting.
                    }
                }
                Feedback::Simulation(sim) => {
                    if !sim.success {
                        warn!(
                            "Simulation failure for test case '{}': expected '{}', got '{}'. Notes: {:?}",
                            sim.test_case, sim.expected_intent, sim.actual_intent, sim.notes
                        );
                        // TODO: Integrate simulation feedback into rules.
                    }
                }
            }
        }
    }
}

impl FeedbackUpdateStrategy for RuleUpdater {
    fn update(&self, feedback_batch: &[Feedback]) {
        self.apply_feedback(feedback_batch);
    }
}

// ================================================================================================
// Model Trainer (ML Stub)
// ================================================================================================

/// Stub for ML model trainer integration.
///
/// Replace with actual ML pipeline hooks or FFI calls when integrating
/// external models or on-device learners.
pub struct ModelTrainer;

impl ModelTrainer {
    pub fn new() -> Self {
        ModelTrainer
    }

    pub fn retrain_models(&self, feedback_batch: &[Feedback]) {
        info!(
            "Retraining ML models with {} feedback items...",
            feedback_batch.len()
        );
        // TODO: Implement training logic or call external ML services.
    }
}

impl FeedbackUpdateStrategy for ModelTrainer {
    fn update(&self, feedback_batch: &[Feedback]) {
        self.retrain_models(feedback_batch);
    }
}

// ================================================================================================
// Async Feedback Ingestion (Tokio)
// ================================================================================================

/// Async feedback ingestion API using Tokio channels for real-time processing.
///
/// This is the primary entry point for UI/API layers to submit feedback
/// without blocking.
pub struct AsyncFeedbackIngestor {
    sender: mpsc::Sender<Feedback>,
}

impl AsyncFeedbackIngestor {
    /// Create a new async ingestor bound to a FeedbackProcessor.
    pub fn new(processor: Arc<FeedbackProcessor>) -> Self {
        let (tx, mut rx) = mpsc::channel::<Feedback>(100);

        tokio::spawn(async move {
            let mut batch: Vec<Feedback> = Vec::new();
            let batch_size = 20;
            let batch_timeout = tokio::time::Duration::from_secs(5);
            let mut timeout = tokio::time::sleep(batch_timeout);
            tokio::pin!(timeout);

            loop {
                tokio::select! {
                    Some(feedback) = rx.recv() => {
                        batch.push(feedback);
                        if batch.len() >= batch_size {
                            processor.process_feedback_batch(batch.drain(..).collect());
                            timeout.as_mut().reset(tokio::time::Instant::now() + batch_timeout);
                        }
                    }
                    _ = &mut timeout => {
                        if !batch.is_empty() {
                            processor.process_feedback_batch(batch.drain(..).collect());
                        }
                        timeout.as_mut().reset(tokio::time::Instant::now() + batch_timeout);
                    }
                }
            }
        });

        AsyncFeedbackIngestor { sender: tx }
    }

    /// Public API to submit feedback asynchronously.
    pub async fn submit_feedback(
        &self,
        feedback: Feedback,
    ) -> Result<(), mpsc::error::SendError<Feedback>> {
        self.sender.send(feedback).await
    }
}

// ================================================================================================
// Telemetry & Safety Integration Helpers
// ================================================================================================

/// Convert a TelemetryEvent into a Feedback item when appropriate.
///
/// Not all telemetry events map cleanly to SystemTelemetry; we focus on
/// routing anomalies and safety decisions as structured feedback.
pub fn feedback_from_telemetry_event(event: TelemetryEvent) -> Option<Feedback> {
    match event {
        TelemetryEvent::Routing { target, confidence, reason } => {
            // Treat low-confidence routing as telemetry feedback.
            let anomaly = if confidence < 0.5 { 0.8 } else { 0.2 };
            Some(Feedback::Telemetry(SystemTelemetry {
                telemetry_id: Uuid::new_v4(),
                intent_label: target,
                error_code: None,
                anomaly_score: anomaly,
                details: Some(format!("Routing reason: {}", reason)),
                timestamp: Utc::now(),
            }))
        }
        TelemetryEvent::SafetyDecision { verdict } => {
            Some(feedback_from_safety_verdict(&verdict))
        }
        TelemetryEvent::EvolutionSummary { total, allowed, blocked } => {
            // Treat high block ratio as an anomaly.
            let anomaly = if total > 0 {
                (blocked as f32 / total as f32).min(1.0)
            } else {
                0.0
            };

            Some(Feedback::Telemetry(SystemTelemetry {
                telemetry_id: Uuid::new_v4(),
                intent_label: "evolution".to_string(),
                error_code: None,
                anomaly_score: anomaly,
                details: Some(format!(
                    "Evolution summary: total={}, allowed={}, blocked={}",
                    total, allowed, blocked
                )),
                timestamp: Utc::now(),
            }))
        }
        TelemetryEvent::Log { level, message } => {
            // Only treat Error logs as telemetry feedback.
            match level {
                TelemetryLevel::Error => Some(Feedback::Telemetry(SystemTelemetry {
                    telemetry_id: Uuid::new_v4(),
                    intent_label: "log".to_string(),
                    error_code: Some("error_log".to_string()),
                    anomaly_score: 0.9,
                    details: Some(message),
                    timestamp: Utc::now(),
                })),
                _ => None,
            }
        }
    }
}

/// Convert a SafetyVerdict into SystemTelemetry-style feedback.
pub fn feedback_from_safety_verdict(verdict: &SafetyVerdict) -> Feedback {
    let (anomaly_score, details) = if verdict.is_forbidden() {
        (
            1.0,
            Some(format!(
                "Forbidden proposal: {}",
                verdict.reason
            )),
        )
    } else if verdict.requires_review() {
        (
            0.7,
            Some(format!(
                "Review-required proposal: {}",
                verdict.reason
            )),
        )
    } else {
        (
            0.2,
            Some(format!(
                "Safe proposal: {}",
                verdict.reason
            )),
        )
    };

    Feedback::Telemetry(SystemTelemetry {
        telemetry_id: Uuid::new_v4(),
        intent_label: "safety".to_string(),
        error_code: None,
        anomaly_score,
        details,
        timestamp: Utc::now(),
    })
}

// ================================================================================================
// Example Flow
// ================================================================================================

/// Example integration function to demonstrate feedback ingestion.
///
/// This is not used in production directly, but serves as a reference
/// for how UI/API layers can wire into the feedback loop.
pub async fn example_feedback_flow(ingestor: &AsyncFeedbackIngestor) {
    let user_feedback = Feedback::User(UserFeedback {
        feedback_id: Uuid::new_v4(),
        intent_label: "browse books".to_string(),
        user_rating: 2,
        comments: Some("The classification was off.".to_string()),
        timestamp: Utc::now(),
    });

    if let Err(e) = ingestor.submit_feedback(user_feedback).await {
        error!("Failed to submit feedback: {:?}", e);
    }
}

// ================================================================================================
// Integration Notes
// ================================================================================================
//
// - After any `Reasoner` produces an `IntentPlan`, surface the result for feedback.
// - Collect explicit feedback via UI, API, or logs and submit it to `AsyncFeedbackIngestor`.
// - Run background tasks that use `FeedbackProcessor` to update rules and models.
// - Use `FeedbackStore` for auditability, explainability, and compliance.
// - Extend `FeedbackSource` and `FeedbackUpdateStrategy` for new channels and strategies.
// - Use `feedback_from_telemetry_event` and `feedback_from_safety_verdict` to bridge
//   telemetry/safety subsystems into the feedback loop.
// ================================================================================================

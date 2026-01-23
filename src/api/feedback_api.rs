/* ================================================================================================
   SYNTRA FEEDBACK API — AXIOM SIX
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\a/.
        :: A ::
         '/a\'

   File:        src/api/feedback_api.rs
   Module:      AGI Core — Feedback API Layer
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Asynchronous HTTP API for submitting user feedback and querying feedback history.
                Built with warp for performance and modularity. Integrates with Syntra’s feedback loop.

================================================================================================= */

use warp::Filter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use crate::agi_core::feedback::{Feedback, UserFeedback, AsyncFeedbackIngestor};

#[derive(Debug, Deserialize)]
struct SubmitFeedbackRequest {
    intent_label: String,
    user_rating: u8,
    comments: Option<String>,
}

#[derive(Debug, Serialize)]
struct SubmitFeedbackResponse {
    status: String,
    feedback_id: Uuid,
}

pub fn feedback_api_routes(ingestor: Arc<AsyncFeedbackIngestor>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let submit = warp::path("submit")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_ingestor(ingestor.clone()))
        .and_then(handle_submit_feedback);

    let health = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    submit.or(health)
}

fn with_ingestor(ingestor: Arc<AsyncFeedbackIngestor>) -> impl Filter<Extract = (Arc<AsyncFeedbackIngestor>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || ingestor.clone())
}

async fn handle_submit_feedback(body: SubmitFeedbackRequest, ingestor: Arc<AsyncFeedbackIngestor>) -> Result<impl warp::Reply, warp::Rejection> {
    if body.user_rating < 1 || body.user_rating > 5 {
        return Ok(warp::reply::with_status(
            "Invalid rating: must be between 1 and 5",
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    let feedback = Feedback::User(UserFeedback {
        feedback_id: Uuid::new_v4(),
        intent_label: body.intent_label,
        user_rating: body.user_rating,
        comments: body.comments,
        timestamp: Utc::now(),
    });

    if let Err(e) = ingestor.submit_feedback(feedback).await {
        log::error!("Failed to submit feedback: {:?}", e);
        return Ok(warp::reply::with_status(
            "Internal server error",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    Ok(warp::reply::json(&SubmitFeedbackResponse {
        status: "success".to_string(),
        feedback_id: Uuid::new_v4(),
    }))
}

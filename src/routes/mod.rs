pub mod health;
pub mod llm;
pub mod metrics;
pub mod rules;
pub mod tasks;
pub mod visualization;

use crate::state::AppState;
use axum::http::StatusCode;
use axum::{
    routing::{delete, get, post},
    Router,
};

pub type ApiResult<T> = Result<T, (StatusCode, String)>;

pub(crate) fn internal_error<E: ToString>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub(crate) fn bad_request<E: ToString>(err: E) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, err.to_string())
}

pub(crate) fn not_found<E: ToString>(err: E) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, err.to_string())
}

pub fn build_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/metrics", get(metrics::get_metrics))
        .route("/metrics/vectorized", get(metrics::get_vectorized_metrics))
        .route("/tasks", get(tasks::list_tasks).post(tasks::create_task))
        .route("/tasks/:id", get(tasks::get_task_status))
        .route("/llm/query", post(llm::llm_query))
        .route("/llm/plan-eqgft-task", post(llm::plan_eqgft_task))
        .route("/llm/research-campaign", post(llm::start_research_campaign))
        .route("/rules", post(rules::register_rule))
        .route("/rules/:name", delete(rules::delete_rule))
        .route("/visualization/packet", get(visualization::get_packet))
        .route(
            "/visualization/hopfion-field",
            get(visualization::get_hopfion_field),
        )
}

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::core::types::GeometricMetrics;
use crate::state::AppState;

use super::{bad_request, not_found, ApiResult};

#[derive(Deserialize)]
pub struct RegisterRuleRequest {
    pub name: String,
    pub delta_v: Option<f64>,
    pub delta_s: Option<f64>,
    pub delta_q: Option<f64>,
}

#[derive(Serialize)]
pub struct RegisterRuleResponse {
    pub registered: bool,
    pub rule_count: usize,
}

pub async fn register_rule(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRuleRequest>,
) -> ApiResult<Json<RegisterRuleResponse>> {
    if payload.name.trim().is_empty() {
        return Err(bad_request("Rule name cannot be empty"));
    }

    let mut engine = state.metric_engine.write().await;
    let name = payload.name.clone();
    engine.register_rule(name.clone(), move |metrics: &mut GeometricMetrics| {
        if let Some(delta) = payload.delta_v {
            metrics.v_geometric += delta;
        }
        if let Some(delta) = payload.delta_s {
            metrics.s_geometric = (metrics.s_geometric + delta).clamp(0.0, 1.0);
        }
        if let Some(delta) = payload.delta_q {
            metrics.q_oscillator += delta;
        }
        metrics.custom_metrics.insert(format!("rule:{}", name), serde_json::json!(1.0));
    });

    let response = RegisterRuleResponse {
        registered: true,
        rule_count: engine.len(),
    };

    Ok(Json(response))
}

pub async fn delete_rule(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> ApiResult<Json<RegisterRuleResponse>> {
    let mut engine = state.metric_engine.write().await;
    let removed = engine.remove_rule(&name);

    if !removed {
        return Err(not_found("Rule not found"));
    }

    let response = RegisterRuleResponse {
        registered: false,
        rule_count: engine.len(),
    };

    Ok(Json(response))
}

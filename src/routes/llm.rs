use axum::{extract::State, Json};
use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::core::types::{GeometricMetrics, GeometricOperator, GeometricTaskCommand};
use crate::state::AppState;

use super::{bad_request, internal_error, ApiResult};

pub async fn plan_eqgft_task(
    State(state): State<AppState>,
    Json(payload): Json<LlmQuery>,
) -> ApiResult<Json<GeometricTaskCommand>> {
    let context = if payload.context.is_null() {
        serde_json::json!({
            "current_metrics": state
                .processor
                .get_metrics()
                .map_err(internal_error)?
        })
    } else {
        payload.context
    };

    let result = state
        .llm_gateway
        .submit_geometric_query(&payload.query, &context)
        .await
        .map_err(|err| bad_request(err.to_string()))?;

    Ok(Json(result))
}

#[derive(Deserialize)]
pub struct LlmQuery {
    pub query: String,
    #[serde(default)]
    pub context: Value,
}

pub async fn llm_query(
    State(state): State<AppState>,
    Json(payload): Json<LlmQuery>,
) -> ApiResult<Json<GeometricTaskCommand>> {
    let context = if payload.context.is_null() {
        serde_json::json!({
            "current_metrics": state
                .processor
                .get_metrics()
                .map_err(internal_error)?
        })
    } else {
        payload.context
    };

    let result = state
        .llm_gateway
        .submit_geometric_query(&payload.query, &context)
        .await
        .map_err(|err| bad_request(err.to_string()))?;

    Ok(Json(result))
}

#[derive(Deserialize)]
pub struct ResearchCampaignRequest {
    pub goal: String,
    #[serde(default = "default_max_steps")]
    pub max_steps: usize,
    pub optimization_target: String,
    pub target_value: Option<f64>,
    #[serde(default)]
    pub context: Value,
}

#[derive(Serialize, Clone)]
pub struct ResearchStepSummary {
    pub step: usize,
    pub task: GeometricTaskCommand,
    pub result_metrics: GeometricMetrics,
    pub improvement: f64,
    pub progress: f64,
}

#[derive(Serialize)]
pub struct ResearchCampaignResponse {
    pub goal: String,
    pub optimization_target: String,
    pub target_value: f64,
    pub completed_steps: usize,
    pub goal_progress: f64,
    pub history: Vec<ResearchStepSummary>,
    pub final_metrics: GeometricMetrics,
}

fn default_max_steps() -> usize {
    5
}

pub async fn start_research_campaign(
    State(state): State<AppState>,
    Json(request): Json<ResearchCampaignRequest>,
) -> ApiResult<Json<ResearchCampaignResponse>> {
    let mut history = Vec::new();
    let mut current_metrics = state
        .processor
        .get_metrics()
        .map_err(internal_error)?;

    let target_value = request
        .target_value
        .unwrap_or_else(|| infer_default_target(&request.optimization_target));

    let mut best_progress = evaluate_research_progress(
        &current_metrics,
        &request.optimization_target,
        target_value,
    );

    for step_idx in 1..=request.max_steps {
        let llm_context = json!({
            "goal": request.goal,
            "optimization_target": request.optimization_target,
            "target_value": target_value,
            "current_metrics": current_metrics,
            "history": history,
            "goal_progress": best_progress,
            "user_context": request.context,
        });

        let query = format!(
            "Design the next geometric operator to move the system toward `{}` focusing on `{}`. Return a single GeometricTaskCommand JSON.",
            request.goal, request.optimization_target
        );

        let mut task_template = match state
            .llm_gateway
            .submit_geometric_query(&query, &llm_context)
            .await
        {
            Ok(task) => task,
            Err(err) => {
                warn!("LLM research step failed ({}). Using fallback command.", err);
                fallback_task_for_target(&request.optimization_target, target_value)
            }
        };

        // ensure campaign steps never collide on task IDs
        task_template.task_id = None;

        let task_clone = task_template.clone();
        let task_id = state
            .processor
            .submit_task(task_template)
            .map_err(|err| bad_request(err.to_string()))?;

        let execution = state
            .processor
            .execute_task(task_id)
            .map_err(|err| internal_error(err.to_string()))?;

        current_metrics = execution.metrics.clone();
        let progress = evaluate_research_progress(
            &current_metrics,
            &request.optimization_target,
            target_value,
        );
        let improvement = (progress - best_progress).max(0.0);
        if progress > best_progress {
            best_progress = progress;
        }

        history.push(ResearchStepSummary {
            step: step_idx,
            task: task_clone,
            result_metrics: current_metrics.clone(),
            improvement,
            progress,
        });

        if progress >= 0.999 {
            break;
        }
    }

    Ok(Json(ResearchCampaignResponse {
        goal: request.goal,
        optimization_target: request.optimization_target,
        target_value,
        completed_steps: history.len(),
        goal_progress: best_progress,
        history,
        final_metrics: current_metrics,
    }))
}

fn infer_default_target(target: &str) -> f64 {
    match target {
        "topological_winding" => 9.0,
        "quaternion_coherence" => 0.9999,
        "emergent_electron_mass" => compute_target_mass(),
        "fine_structure_constant" => 1.0 / 137.035_999_084,
        _ => 1.0,
    }
}

fn compute_target_mass() -> f64 {
    crate::state::compute_electron_mass()
}

fn evaluate_research_progress(
    metrics: &GeometricMetrics,
    optimization_target: &str,
    target_value: f64,
) -> f64 {
    let current = match optimization_target {
        "topological_winding" => metrics.topological_winding,
        "quaternion_coherence" => metrics.quaternion_coherence,
        "emergent_electron_mass" => metrics.emergent_electron_mass,
        "fine_structure_constant" => metrics.fine_structure_constant,
        "q_oscillator" => metrics.q_oscillator,
        "v_geometric" => metrics.v_geometric,
        "s_geometric" => metrics.s_geometric,
        _ => metrics.v_geometric,
    };

    let denominator = target_value.abs().max(1e-6);
    let distance = (target_value - current).abs();
    (1.0 - (distance / denominator)).clamp(0.0, 1.0)
}

fn fallback_task_for_target(target: &str, target_value: f64) -> GeometricTaskCommand {
    match target {
        "topological_winding" | "q_oscillator" => GeometricTaskCommand {
            task_name: "Fallback Zitterbewegung tuning".into(),
            geometric_operator: GeometricOperator::Zitterbewegung,
            target_module: "sys6_resonator".into(),
            parameters: json!({ "frequency_scale": target_value / 9.0 }),
            expected_output_metric: target.into(),
            task_id: None,
        },
        "quaternion_coherence" | "v_geometric" => GeometricTaskCommand {
            task_name: "Fallback Quaternion coherence".into(),
            geometric_operator: GeometricOperator::QuaternionRotation,
            target_module: "sys7_core".into(),
            parameters: json!({ "theta": 0.25, "axis": [0.0, 1.0, 0.0] }),
            expected_output_metric: target.into(),
            task_id: None,
        },
        "emergent_electron_mass" => GeometricTaskCommand {
            task_name: "Fallback mass adjustment".into(),
            geometric_operator: GeometricOperator::Zitterbewegung,
            target_module: "sys6_resonator".into(),
            parameters: json!({ "frequency_scale": 1.0 }),
            expected_output_metric: target.into(),
            task_id: None,
        },
        "fine_structure_constant" => GeometricTaskCommand {
            task_name: "Fallback α tuning".into(),
            geometric_operator: GeometricOperator::QuaternionRotation,
            target_module: "sys7_alpha".into(),
            parameters: json!({ "theta": 0.1 }),
            expected_output_metric: target.into(),
            task_id: None,
        },
        _ => GeometricTaskCommand {
            task_name: "Fallback geometric derivation".into(),
            geometric_operator: GeometricOperator::GeometricDerivation,
            target_module: "sys5_topology".into(),
            parameters: json!({ "delta": 0.01 }),
            expected_output_metric: target.into(),
            task_id: None,
        },
    }
}

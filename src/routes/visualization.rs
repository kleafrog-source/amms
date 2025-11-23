use axum::{extract::State, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::core::types::SemanticAnchor;
use crate::state::AppState;
use crate::visualization::protocol::VisualizationPacket;

use super::{internal_error, ApiResult};

#[derive(Serialize)]
pub struct VisualizationResponse {
    pub packet: VisualizationPacket,
}

pub async fn get_packet(State(state): State<AppState>) -> ApiResult<Json<VisualizationResponse>> {
    let metrics = state.processor.get_metrics().map_err(internal_error)?;

    let anchors = vec![SemanticAnchor {
        id: Uuid::new_v4(),
        name: "Root Anchor".into(),
        description: "Placeholder semantic anchor".into(),
        position: [0.0, 0.0, 0.0, 1.0],
        metadata: serde_json::json!({ "note": "replace with real anchors" }),
    }];

    let packet = VisualizationPacket::new(metrics, anchors);

    Ok(Json(VisualizationResponse { packet }))
}

#[cfg(feature = "eqgft")]
pub async fn get_hopfion_field(
    State(state): State<AppState>,
) -> ApiResult<Json<Option<mmss_eqgft::HopfionSolitonField>>> {
    let field = state.processor.get_hopfion_field().map_err(internal_error)?;
    Ok(Json(field))
}

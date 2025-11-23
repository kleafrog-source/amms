use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Geometric operators for the MMSS system
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GeometricOperator {
    /// Quaternion rotation operator (⟲Q)
    QuaternionRotation,
    /// Zitterbewegung operator (⥁Z)
    Zitterbewegung,
    /// Geometric derivation operator (⇛G)
    GeometricDerivation,
    /// Semantic synthesis operator (⥂S)
    SemanticSynthesis,
    /// Simulate EQGFT asymmetry
    SimulateEqgftAsymmetry,
    /// Generate Hopfion field
    GenerateHopfionField,
    /// Execute custom Python script
    CustomPythonScript,
}

/// Geometric task command structure for LLM interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometricTaskCommand {
    /// Brief description of the task
    pub task_name: String,
    /// Main geometric operator to apply
    pub geometric_operator: GeometricOperator,
    /// Target module in the Pure Logic system
    pub target_module: String,
    /// Parameters required for task execution
    pub parameters: serde_json::Value,
    /// Expected output metric to monitor
    pub expected_output_metric: String,
    /// Optional task ID for tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<Uuid>,
}

/// Quaternion type for geometric operations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Geometric metrics for system monitoring
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeometricMetrics {
    /// Geometric volume metric
    pub v_geometric: f64,
    /// Geometric stability metric
    pub s_geometric: f64,
    /// Oscillator quality factor
    pub q_oscillator: f64,
    /// Quaternion coherence (SYS7)
    #[serde(default)]
    pub quaternion_coherence: f64,
    /// Emergent electron mass from zitterbewegung
    #[serde(default)]
    pub emergent_electron_mass: f64,
    /// Fine structure constant derived from geometry
    #[serde(default)]
    pub fine_structure_constant: f64,
    /// Zitterbewegung entropy (SYS6)
    #[serde(default)]
    pub zitterbewegung_entropy: f64,
    /// Topological winding number (SYS5)
    #[serde(default)]
    pub topological_winding: f64,
    /// Additional custom metrics
    pub custom_metrics: HashMap<String, serde_json::Value>,
}

/// Semantic anchor for linguistic elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAnchor {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub position: [f64; 4], // 4D position for quaternion space
    pub metadata: serde_json::Value,
}

/// Task execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskExecutionResult {
    pub task_id: Uuid,
    pub success: bool,
    pub metrics: GeometricMetrics,
    pub output: serde_json::Value,
    pub error: Option<String>,
}

/// System state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub state_id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metrics: GeometricMetrics,
    pub active_anchors: Vec<SemanticAnchor>,
    pub active_tasks: Vec<Uuid>,
}

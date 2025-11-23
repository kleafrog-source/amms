use crate::core::types::{GeometricMetrics, GeometricOperator, Quaternion};
use crate::state::{
    compute_electron_mass, compute_fine_structure, compute_quaternion_coherence, compute_zitter_entropy,
    C, HBAR, ZITTER_AMPLITUDE,
};
use serde_json::Value;
use std::collections::HashMap;

/// Simple placeholder for emergence logic parameters.
#[derive(Debug, Clone)]
pub struct EmergenceConfig {
    pub step_size: f64,
}

fn normalize_axis(arr: &[Value]) -> Option<[f64; 3]> {
    if arr.len() < 3 {
        return None;
    }

    let x = arr.get(0).and_then(Value::as_f64)?;
    let y = arr.get(1).and_then(Value::as_f64)?;
    let z = arr.get(2).and_then(Value::as_f64)?;
    Some([x, y, z])
}

impl EmergenceLogic {
    fn baseline_metrics() -> GeometricMetrics {
        let coherence = compute_quaternion_coherence();
        let entropy = compute_zitter_entropy();
        let electron_mass = compute_electron_mass();
        let fine_structure = compute_fine_structure();
        let default_winding = 8.9997;

        GeometricMetrics {
            v_geometric: coherence,
            s_geometric: entropy,
            q_oscillator: default_winding,
            quaternion_coherence: coherence,
            emergent_electron_mass: electron_mass,
            fine_structure_constant: fine_structure,
            zitterbewegung_entropy: entropy,
            topological_winding: default_winding,
            custom_metrics: HashMap::new(),
        }
    }
}

impl Default for EmergenceConfig {
    fn default() -> Self {
        Self { step_size: 0.01 }
    }
}

/// Basic SYS7-SYS1 cascade placeholder.
#[derive(Debug, Clone)]
pub struct EmergenceLogic {
    config: EmergenceConfig,
    metrics: GeometricMetrics,
}

impl EmergenceLogic {
    pub fn new(config: Option<EmergenceConfig>) -> Self {
        Self {
            config: config.unwrap_or_default(),
            metrics: Self::baseline_metrics(),
        }
    }

    pub fn apply_operator(&mut self, op: GeometricOperator, params: &Value) -> &GeometricMetrics {
        let magnitude = extract_scalar(params).unwrap_or(1.0);

        match op {
            GeometricOperator::QuaternionRotation => {
                let theta = params
                    .get("theta")
                    .and_then(Value::as_f64)
                    .unwrap_or(magnitude);
                let axis = params
                    .get("axis")
                    .and_then(Value::as_array)
                    .and_then(|arr| normalize_axis(arr))
                    .unwrap_or([0.0, 1.0, 0.0]);

                let axis_norm = (axis[0].powi(2) + axis[1].powi(2) + axis[2].powi(2)).sqrt();
                let coherence_boost = (theta * 0.5).sin().abs() * 0.005 * axis_norm.max(1e-6);

                self.metrics.quaternion_coherence = (self.metrics.quaternion_coherence + coherence_boost)
                    .clamp(0.0, 0.9999);
                self.metrics.v_geometric = self.metrics.quaternion_coherence;
            }
            GeometricOperator::Zitterbewegung => {
                let freq_scale = params
                    .get("frequency_scale")
                    .and_then(Value::as_f64)
                    .unwrap_or(magnitude.abs());
                let scaled_amplitude = (ZITTER_AMPLITUDE / freq_scale.max(1e-6)).abs();

                self.metrics.emergent_electron_mass = HBAR / (2.0 * C * scaled_amplitude);
                self.metrics.topological_winding =
                    (self.metrics.topological_winding + (freq_scale - 1.0) * 0.0001).max(0.0);
                self.metrics.q_oscillator = self.metrics.topological_winding.max(0.0);
            }
            GeometricOperator::GeometricDerivation => {
                let delta = params
                    .get("delta")
                    .and_then(Value::as_f64)
                    .unwrap_or(magnitude);
                self.metrics.s_geometric = (self.metrics.s_geometric + delta * 0.001).clamp(0.0001, 1.0);
                self.metrics.zitterbewegung_entropy = self.metrics.s_geometric;
            }
            GeometricOperator::SemanticSynthesis => {
                let coherence_hint = params
                    .get("coherence_hint")
                    .and_then(Value::as_f64)
                    .unwrap_or(0.95);
                let anchor_name = params
                    .get("anchor")
                    .and_then(Value::as_str)
                    .unwrap_or("quantum-atom");

                let semantic_strength =
                    (self.metrics.quaternion_coherence * coherence_hint * 10.0).max(0.0);
                self.metrics
                    .custom_metrics
                    .insert(format!("anchor:{}", anchor_name), serde_json::json!(semantic_strength));
            }
            _ => {}
        }

        self.metrics.fine_structure_constant =
            (compute_fine_structure() / self.metrics.quaternion_coherence.max(1e-6)).min(1.0);
        if self.metrics.zitterbewegung_entropy <= 0.0 {
            self.metrics.zitterbewegung_entropy = compute_zitter_entropy();
        }
        if self.metrics.emergent_electron_mass <= 0.0 {
            self.metrics.emergent_electron_mass = compute_electron_mass();
        }
        if self.metrics.quaternion_coherence <= 0.0 {
            self.metrics.quaternion_coherence = compute_quaternion_coherence();
        }
        if self.metrics.topological_winding <= 0.0 {
            self.metrics.topological_winding = self.metrics.q_oscillator;
        }

        &self.metrics
    }

    pub fn integrate_quaternion(&mut self, q: Quaternion) -> &GeometricMetrics {
        self.metrics.custom_metrics.insert("q_w".to_string(), serde_json::json!(q.w));
        self.metrics.custom_metrics.insert("q_x".to_string(), serde_json::json!(q.x));
        self.metrics.custom_metrics.insert("q_y".to_string(), serde_json::json!(q.y));
        self.metrics.custom_metrics.insert("q_z".to_string(), serde_json::json!(q.z));
        &self.metrics
    }

    pub fn metrics(&self) -> &GeometricMetrics {
        &self.metrics
    }
}

fn extract_scalar(params: &Value) -> Option<f64> {
    if let Some(val) = params.as_f64() {
        return Some(val);
    }

    if let Some(obj) = params.as_object() {
        for key in ["magnitude", "value", "amount", "scale"] {
            if let Some(v) = obj.get(key).and_then(Value::as_f64) {
                return Some(v);
            }
        }
    }

    None
}

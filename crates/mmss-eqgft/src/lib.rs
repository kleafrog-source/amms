use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EqgftConfig {
    pub n_events: u64,
    pub kappa: f64,
    pub systematic_error: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolarizationAsymmetry {
    pub a: f64,
    pub kappa: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HopfionSolitonField {
    pub q_x: Vec<[f64; 4]>,
    pub n_h: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SensitivityCurve {
    pub sigma: Vec<f64>,
    pub n_values: Vec<u64>,
}

pub fn calculate_polarization_asymmetry(kappa: f64) -> PolarizationAsymmetry {
    PolarizationAsymmetry { a: kappa, kappa }
}

pub fn generate_hopfion_soliton_field() -> HopfionSolitonField {
    HopfionSolitonField {
        q_x: vec![[1.0, 0.0, 0.0, 0.0]], // Placeholder
        n_h: 1,
    }
}

pub fn calculate_sensitivity_curve(a: f64, n_values: Vec<u64>) -> SensitivityCurve {
    let sigma = n_values
        .iter()
        .map(|&n| a.abs() / ((1.0 - a.powi(2)) / n as f64).sqrt())
        .collect();
    SensitivityCurve { sigma, n_values }
}

pub fn execute_python_script(script: &str) -> PyResult<(String, String)> {
    Python::with_gil(|py| {
        let result = py.eval(script, None, None)?;
        let json_output = result.getattr("json_output")?.to_string();
        let png_path = result.getattr("png_path")?.to_string();
        Ok((json_output, png_path))
    })
}

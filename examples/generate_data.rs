
#[cfg(feature = "eqgft")]
use mmss_eqgft::{
    calculate_polarization_asymmetry, calculate_sensitivity_curve, generate_hopfion_soliton_field,
};
use mmss_core::export::arrow::write_records_to_file;
use mmss_core::structex_bridge::{MmssRecord, PatternMatcher};
use serde_json::json;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = (0..100)
        .map(|i| {
            let metric_type = match i % 4 {
                0 => "cpu",
                1 => "memory",
                2 => "network",
                _ => "disk",
            };
            MmssRecord {
                id: i as u64,
                kind: metric_type.to_string(),
                timestamp: 1732400000 + (i as i64 * 60),
                payload: json!({
                    "value": rand::random::<f64>() * 100.0,
                    "unit": if metric_type == "network" { "MB/s" } else { "%" },
                    "host": format!("host-{}", rand::random::<u8>() % 5 + 1),
                }),
            }
        })
        .collect::<Vec<_>>();

    write_records_to_file(Path::new("data.arrow"), &records)?;

    #[cfg(feature = "eqgft")]
    {
        let asymmetry = calculate_polarization_asymmetry(0.2);
        let hopfion_field = generate_hopfion_soliton_field();
        let sensitivity_curve =
            calculate_sensitivity_curve(asymmetry.a, (1..=200000).step_by(1000).collect());

        let eqgft_records = vec![
            MmssRecord {
                id: 0,
                kind: "eqgft_asymmetry".to_string(),
                timestamp: 1732400000,
                payload: json!(asymmetry),
            },
            MmssRecord {
                id: 1,
                kind: "eqgft_hopfion_field".to_string(),
                timestamp: 1732400000,
                payload: json!(hopfion_field),
            },
            MmssRecord {
                id: 2,
                kind: "eqgft_sensitivity_curve".to_string(),
                timestamp: 1732400000,
                payload: json!(sensitivity_curve),
            },
        ];

        write_records_to_file(Path::new("eqgft_data.arrow"), &eqgft_records)?;
    }

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
pub struct Attenuation {
    curve_type: crate::types::AttenuationType,
    #[serde(default = "default_tuning_parameter")]
    tuning_parameter: f32,
}
fn default_tuning_parameter() -> f32 {
    -1.0
}

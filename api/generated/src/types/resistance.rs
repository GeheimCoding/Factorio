#[derive(Debug, serde::Deserialize)]
pub struct Resistance {
    #[serde(default = "default_decrease")]
    decrease: f32,
    #[serde(default = "default_percent")]
    percent: f32,
    #[serde(rename = "type")]
    type_: crate::types::DamageTypeID,
}
fn default_decrease() -> f32 {
    0.0
}
fn default_percent() -> f32 {
    0.0
}

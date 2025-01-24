#[derive(serde::Deserialize)]
pub struct TurretSpecialEffect {
    attacking_falloff: Option<f32>,
    attacking_max_radius: Option<f32>,
    attacking_min_radius: Option<f32>,
    center: Option<crate::types::TurretSpecialEffectCenter>,
    #[serde(default = "default_falloff")]
    falloff: f32,
    max_radius: Option<f32>,
    #[serde(default = "default_min_radius")]
    min_radius: f32,
    #[serde(rename = "type")]
    type_: String,
}
fn default_falloff() -> f32 {
    0.0
}
fn default_min_radius() -> f32 {
    0.0
}

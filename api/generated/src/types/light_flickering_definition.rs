#[derive(Debug, serde::Deserialize)]
pub struct LightFlickeringDefinition {
    #[serde(default = "default_border_fix_speed")]
    border_fix_speed: f32,
    // default: {r=1, g=1, b=1} (White)
    color: Option<crate::types::Color>,
    #[serde(default = "default_derivation_change_deviation")]
    derivation_change_deviation: f32,
    #[serde(default = "default_derivation_change_frequency")]
    derivation_change_frequency: f32,
    #[serde(default = "default_light_intensity_to_size_coefficient")]
    light_intensity_to_size_coefficient: f32,
    #[serde(default = "default_maximum_intensity")]
    maximum_intensity: f32,
    #[serde(default = "default_minimum_intensity")]
    minimum_intensity: f32,
    #[serde(default = "default_minimum_light_size")]
    minimum_light_size: f32,
}
fn default_border_fix_speed() -> f32 {
    0.0
}
fn default_derivation_change_deviation() -> f32 {
    0.1
}
fn default_derivation_change_frequency() -> f32 {
    0.3
}
fn default_light_intensity_to_size_coefficient() -> f32 {
    0.5
}
fn default_maximum_intensity() -> f32 {
    0.8
}
fn default_minimum_intensity() -> f32 {
    0.2
}
fn default_minimum_light_size() -> f32 {
    0.5
}

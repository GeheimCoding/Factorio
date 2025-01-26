#[derive(Debug, serde::Deserialize)]
pub struct AgriculturalCraneSpeedGrappler {
    #[serde(default = "default_allow_transpolar_movement")]
    allow_transpolar_movement: bool,
    #[serde(default = "default_extension_speed")]
    extension_speed: f64,
    #[serde(default = "default_horizontal_turn_rate")]
    horizontal_turn_rate: f64,
    #[serde(default = "default_vertical_turn_rate")]
    vertical_turn_rate: f64,
}
fn default_allow_transpolar_movement() -> bool {
    true
}
fn default_extension_speed() -> f64 {
    0.0
}
fn default_horizontal_turn_rate() -> f64 {
    0.0
}
fn default_vertical_turn_rate() -> f64 {
    0.0
}

#[derive(serde::Deserialize)]
pub struct AgriculturalCraneSpeedArm {
    #[serde(default = "default_extension_speed")]
    extension_speed: f64,
    #[serde(default = "default_turn_rate")]
    turn_rate: f64,
}
fn default_extension_speed() -> f64 {
    0.1
}
fn default_turn_rate() -> f64 {
    0.0
}

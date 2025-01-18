#[derive(serde::Deserialize)]
pub struct ActivityMatchingModifiers {
    #[serde(default = "default_inverted")]
    inverted: bool,
    // default: infinity
    maximum: f32,
    #[serde(default = "default_minimum")]
    minimum: f32,
    #[serde(default = "default_multiplier")]
    multiplier: f32,
    #[serde(default = "default_offset")]
    offset: f32,
}
fn default_inverted() -> bool {
    false
}
fn default_minimum() -> f32 {
    0.0
}
fn default_multiplier() -> f32 {
    1.0
}
fn default_offset() -> f32 {
    0.0
}

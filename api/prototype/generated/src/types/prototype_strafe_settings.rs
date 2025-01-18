#[derive(serde::Deserialize)]
pub struct PrototypeStrafeSettings {
    #[serde(default = "default_clockwise_chance")]
    clockwise_chance: f32,
    #[serde(default = "default_face_target")]
    face_target: bool,
    #[serde(default = "default_ideal_distance")]
    ideal_distance: f64,
    #[serde(default = "default_ideal_distance_importance")]
    ideal_distance_importance: f32,
    #[serde(default = "default_ideal_distance_importance_variance")]
    ideal_distance_importance_variance: f32,
    #[serde(default = "default_ideal_distance_tolerance")]
    ideal_distance_tolerance: f64,
    #[serde(default = "default_ideal_distance_variance")]
    ideal_distance_variance: f64,
    #[serde(default = "default_max_distance")]
    max_distance: f64,
}
fn default_clockwise_chance() -> f32 {
    0.5
}
fn default_face_target() -> bool {
    false
}
fn default_ideal_distance() -> f64 {
    10.0
}
fn default_ideal_distance_importance() -> f32 {
    0.5
}
fn default_ideal_distance_importance_variance() -> f32 {
    0.1
}
fn default_ideal_distance_tolerance() -> f64 {
    0.5
}
fn default_ideal_distance_variance() -> f64 {
    1.0
}
fn default_max_distance() -> f64 {
    20.0
}

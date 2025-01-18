#[derive(serde::Deserialize)]
pub struct CaptureRobotPrototype {
    base_: crate::prototypes::FlyingRobotPrototype,
    capture_animation: crate::types::Animation,
    #[serde(default = "default_capture_speed")]
    capture_speed: f64,
    destroy_action: crate::types::Trigger,
    #[serde(default = "default_search_radius")]
    search_radius: f64,
}
fn default_capture_speed() -> f64 {
    1.0
}
fn default_search_radius() -> f64 {
    1.0
}

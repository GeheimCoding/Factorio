#[derive(serde::Deserialize)]
pub struct CaptureRobotPrototype {
    base_: crate::prototypes::FlyingRobotPrototype,
    capture_animation: crate::types::Animation,
    capture_speed: f64,
    destroy_action: crate::types::Trigger,
    search_radius: f64,
}

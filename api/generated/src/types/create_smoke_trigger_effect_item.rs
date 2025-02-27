#[derive(Debug, serde::Deserialize)]
pub struct CreateSmokeTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::CreateEntityTriggerEffectItem,
    #[serde(default = "default_initial_height")]
    initial_height: f32,
    speed: Option<crate::types::Vector>,
    #[serde(default = "default_speed_from_center")]
    speed_from_center: f32,
    #[serde(default = "default_speed_from_center_deviation")]
    speed_from_center_deviation: f32,
    #[serde(default = "default_speed_multiplier")]
    speed_multiplier: f32,
    #[serde(default = "default_speed_multiplier_deviation")]
    speed_multiplier_deviation: f32,
    #[serde(default = "default_starting_frame")]
    starting_frame: f32,
    #[serde(default = "default_starting_frame_deviation")]
    starting_frame_deviation: f32,
}
fn default_initial_height() -> f32 {
    0.0
}
fn default_speed_from_center() -> f32 {
    0.0
}
fn default_speed_from_center_deviation() -> f32 {
    0.0
}
fn default_speed_multiplier() -> f32 {
    0.0
}
fn default_speed_multiplier_deviation() -> f32 {
    0.0
}
fn default_starting_frame() -> f32 {
    0.0
}
fn default_starting_frame_deviation() -> f32 {
    0.0
}

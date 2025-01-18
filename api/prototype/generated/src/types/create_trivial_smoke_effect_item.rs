#[derive(serde::Deserialize)]
pub struct CreateTrivialSmokeEffectItem {
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_initial_height")]
    initial_height: f32,
    #[serde(default = "default_max_radius")]
    max_radius: f32,
    offset_deviation: crate::types::BoundingBox,
    offsets: Vec<crate::types::Vector>,
    smoke_name: crate::types::TrivialSmokeID,
    speed: crate::types::Vector,
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
    #[serde(rename = "type")]
    type_: String,
}
fn default_initial_height() -> f32 {
    0.0
}
fn default_max_radius() -> f32 {
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

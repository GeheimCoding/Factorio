#[derive(serde::Deserialize)]
pub struct ParticleSourcePrototype {
    base_: crate::prototypes::EntityPrototype,
    height: f32,
    #[serde(default = "default_height_deviation")]
    height_deviation: f32,
    horizontal_speed: f32,
    #[serde(default = "default_horizontal_speed_deviation")]
    horizontal_speed_deviation: f32,
    particle: Option<crate::types::ParticleID>,
    smoke: Option<Vec<crate::types::SmokeSource>>,
    time_before_start: f32,
    #[serde(default = "default_time_before_start_deviation")]
    time_before_start_deviation: f32,
    time_to_live: f32,
    #[serde(default = "default_time_to_live_deviation")]
    time_to_live_deviation: f32,
    vertical_speed: f32,
    #[serde(default = "default_vertical_speed_deviation")]
    vertical_speed_deviation: f32,
}
fn default_height_deviation() -> f32 {
    0.0
}
fn default_horizontal_speed_deviation() -> f32 {
    0.0
}
fn default_time_before_start_deviation() -> f32 {
    0.0
}
fn default_time_to_live_deviation() -> f32 {
    0.0
}
fn default_vertical_speed_deviation() -> f32 {
    0.0
}

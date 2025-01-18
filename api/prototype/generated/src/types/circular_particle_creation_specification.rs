#[derive(serde::Deserialize)]
pub struct CircularParticleCreationSpecification {
    // default: `{0, 0}`
    center: crate::types::Vector,
    #[serde(default = "default_creation_distance")]
    creation_distance: f64,
    #[serde(default = "default_creation_distance_orientation")]
    creation_distance_orientation: f64,
    #[serde(default = "default_direction")]
    direction: f32,
    #[serde(default = "default_direction_deviation")]
    direction_deviation: f32,
    #[serde(default = "default_height")]
    height: f32,
    #[serde(default = "default_height_deviation")]
    height_deviation: f32,
    name: crate::types::ParticleID,
    #[serde(default = "default_speed")]
    speed: f32,
    #[serde(default = "default_speed_deviation")]
    speed_deviation: f32,
    starting_frame_speed: f32,
    #[serde(default = "default_starting_frame_speed_deviation")]
    starting_frame_speed_deviation: f32,
    #[serde(default = "default_use_source_position")]
    use_source_position: bool,
    #[serde(default = "default_vertical_speed")]
    vertical_speed: f32,
    #[serde(default = "default_vertical_speed_deviation")]
    vertical_speed_deviation: f32,
}
fn default_creation_distance() -> f64 {
    0.0
}
fn default_creation_distance_orientation() -> f64 {
    0.0
}
fn default_direction() -> f32 {
    0.2
}
fn default_direction_deviation() -> f32 {
    0.0
}
fn default_height() -> f32 {
    1.0
}
fn default_height_deviation() -> f32 {
    0.0
}
fn default_speed() -> f32 {
    0.1
}
fn default_speed_deviation() -> f32 {
    0.0
}
fn default_starting_frame_speed_deviation() -> f32 {
    0.0
}
fn default_use_source_position() -> bool {
    false
}
fn default_vertical_speed() -> f32 {
    0.0
}
fn default_vertical_speed_deviation() -> f32 {
    0.0
}

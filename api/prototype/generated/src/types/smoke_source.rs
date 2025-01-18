#[derive(serde::Deserialize)]
pub struct SmokeSource {
    deviation: crate::types::Vector,
    // default: Value of `position` rotated east
    east_position: crate::types::Vector,
    frequency: f32,
    #[serde(default = "default_has_8_directions")]
    has_8_directions: bool,
    #[serde(default = "default_height")]
    height: f32,
    #[serde(default = "default_height_deviation")]
    height_deviation: f32,
    name: crate::types::TrivialSmokeID,
    // default: Value of `position` rotated north-east
    north_east_position: crate::types::Vector,
    // default: Value of `position`
    north_position: crate::types::Vector,
    // default: Value of `position` rotated north-west
    north_west_position: crate::types::Vector,
    #[serde(default = "default_offset")]
    offset: f32,
    position: crate::types::Vector,
    // default: Value of `position` rotated south-east
    south_east_position: crate::types::Vector,
    // default: Value of `position` rotated south
    south_position: crate::types::Vector,
    // default: Value of `position` rotated south-west
    south_west_position: crate::types::Vector,
    #[serde(default = "default_starting_frame")]
    starting_frame: u16,
    #[serde(default = "default_starting_frame_deviation")]
    starting_frame_deviation: u16,
    #[serde(default = "default_starting_vertical_speed")]
    starting_vertical_speed: f32,
    #[serde(default = "default_starting_vertical_speed_deviation")]
    starting_vertical_speed_deviation: f32,
    #[serde(default = "default_vertical_speed_slowdown")]
    vertical_speed_slowdown: f32,
    // default: Value of `position` rotated west
    west_position: crate::types::Vector,
}
fn default_has_8_directions() -> bool {
    false
}
fn default_height() -> f32 {
    0.0
}
fn default_height_deviation() -> f32 {
    0.0
}
fn default_offset() -> f32 {
    0.0
}
fn default_starting_frame() -> u16 {
    0
}
fn default_starting_frame_deviation() -> u16 {
    0
}
fn default_starting_vertical_speed() -> f32 {
    0.0
}
fn default_starting_vertical_speed_deviation() -> f32 {
    0.0
}
fn default_vertical_speed_slowdown() -> f32 {
    1.0
}

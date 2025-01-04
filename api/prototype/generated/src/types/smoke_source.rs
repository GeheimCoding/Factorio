pub struct SmokeSource {
    deviation: crate::types::Vector,
    east_position: crate::types::Vector,
    frequency: f32,
    has_8_directions: bool,
    height: f32,
    height_deviation: f32,
    name: crate::types::TrivialSmokeID,
    north_east_position: crate::types::Vector,
    north_position: crate::types::Vector,
    north_west_position: crate::types::Vector,
    offset: f32,
    position: crate::types::Vector,
    south_east_position: crate::types::Vector,
    south_position: crate::types::Vector,
    south_west_position: crate::types::Vector,
    starting_frame: u16,
    starting_frame_deviation: u16,
    starting_vertical_speed: f32,
    starting_vertical_speed_deviation: f32,
    vertical_speed_slowdown: f32,
    west_position: crate::types::Vector,
}

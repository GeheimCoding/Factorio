#[derive(serde::Deserialize)]
pub struct CircularParticleCreationSpecification {
    center: crate::types::Vector,
    creation_distance: f64,
    creation_distance_orientation: f64,
    direction: f32,
    direction_deviation: f32,
    height: f32,
    height_deviation: f32,
    name: crate::types::ParticleID,
    speed: f32,
    speed_deviation: f32,
    starting_frame_speed: f32,
    starting_frame_speed_deviation: f32,
    use_source_position: bool,
    vertical_speed: f32,
    vertical_speed_deviation: f32,
}

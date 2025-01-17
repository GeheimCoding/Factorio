#[derive(serde::Deserialize)]
pub struct ParticleSourcePrototype {
    base_: crate::prototypes::EntityPrototype,
    height: f32,
    height_deviation: f32,
    horizontal_speed: f32,
    horizontal_speed_deviation: f32,
    particle: crate::types::ParticleID,
    smoke: Vec<crate::types::SmokeSource>,
    time_before_start: f32,
    time_before_start_deviation: f32,
    time_to_live: f32,
    time_to_live_deviation: f32,
    vertical_speed: f32,
    vertical_speed_deviation: f32,
}

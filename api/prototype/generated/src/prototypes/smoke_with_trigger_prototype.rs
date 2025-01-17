#[derive(serde::Deserialize)]
pub struct SmokeWithTriggerPrototype {
    base_: crate::prototypes::SmokePrototype,
    action: crate::types::Trigger,
    action_cooldown: u32,
    attach_to_target: bool,
    fade_when_attachment_is_destroyed: bool,
    particle_count: u8,
    particle_distance_scale_factor: f32,
    particle_duration_variation: u32,
    particle_scale_factor: crate::types::Vector,
    particle_spread: crate::types::Vector,
    spread_duration_variation: u32,
    wave_distance: crate::types::Vector,
    wave_speed: crate::types::Vector,
}

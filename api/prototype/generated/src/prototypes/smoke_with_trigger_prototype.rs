#[derive(Debug, serde::Deserialize)]
pub struct SmokeWithTriggerPrototype {
    base_: crate::prototypes::SmokePrototype,
    action: Option<crate::types::Trigger>,
    #[serde(default = "default_action_cooldown")]
    action_cooldown: u32,
    #[serde(default = "default_attach_to_target")]
    attach_to_target: bool,
    #[serde(default = "default_fade_when_attachment_is_destroyed")]
    fade_when_attachment_is_destroyed: bool,
    #[serde(default = "default_particle_count")]
    particle_count: u8,
    #[serde(default = "default_particle_distance_scale_factor")]
    particle_distance_scale_factor: f32,
    #[serde(default = "default_particle_duration_variation")]
    particle_duration_variation: u32,
    particle_scale_factor: Option<crate::types::Vector>,
    particle_spread: Option<crate::types::Vector>,
    #[serde(default = "default_spread_duration_variation")]
    spread_duration_variation: u32,
    wave_distance: Option<crate::types::Vector>,
    wave_speed: Option<crate::types::Vector>,
}
fn default_action_cooldown() -> u32 {
    0
}
fn default_attach_to_target() -> bool {
    false
}
fn default_fade_when_attachment_is_destroyed() -> bool {
    false
}
fn default_particle_count() -> u8 {
    1
}
fn default_particle_distance_scale_factor() -> f32 {
    0.0
}
fn default_particle_duration_variation() -> u32 {
    0
}
fn default_spread_duration_variation() -> u32 {
    0
}

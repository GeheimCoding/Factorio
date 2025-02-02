#[derive(Debug, serde::Deserialize)]
pub struct FluidStreamPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityPrototype,
    action: Option<crate::types::Trigger>,
    ground_light: Option<crate::types::LightDefinition>,
    initial_action: Option<crate::types::Trigger>,
    #[serde(default = "default_oriented_particle")]
    oriented_particle: bool,
    particle: Option<crate::types::Animation>,
    #[serde(default = "default_particle_alpha_per_part")]
    particle_alpha_per_part: f32,
    #[serde(default = "default_particle_buffer_size")]
    particle_buffer_size: u32,
    #[serde(default = "default_particle_end_alpha")]
    particle_end_alpha: f32,
    #[serde(default = "default_particle_fade_out_duration")]
    particle_fade_out_duration: u16,
    #[serde(default = "default_particle_fade_out_threshold")]
    particle_fade_out_threshold: f32,
    particle_horizontal_speed: f32,
    particle_horizontal_speed_deviation: f32,
    #[serde(default = "default_particle_loop_exit_threshold")]
    particle_loop_exit_threshold: f32,
    #[serde(default = "default_particle_loop_frame_count")]
    particle_loop_frame_count: u16,
    #[serde(default = "default_particle_scale_per_part")]
    particle_scale_per_part: f32,
    particle_spawn_interval: u16,
    // default: 4 * `particle_spawn_interval`
    particle_spawn_timeout: Option<u16>,
    #[serde(default = "default_particle_start_alpha")]
    particle_start_alpha: f32,
    #[serde(default = "default_particle_start_scale")]
    particle_start_scale: f32,
    particle_vertical_acceleration: f32,
    #[serde(default = "default_progress_to_create_smoke")]
    progress_to_create_smoke: f32,
    shadow: Option<crate::types::Animation>,
    #[serde(default = "default_shadow_scale_enabled")]
    shadow_scale_enabled: bool,
    smoke_sources: Option<crate::vec::Vec<crate::types::SmokeSource>>,
    special_neutral_target_damage: Option<crate::types::DamageParameters>,
    spine_animation: Option<crate::types::Animation>,
    stream_light: Option<crate::types::LightDefinition>,
    #[serde(default = "default_target_position_deviation")]
    target_position_deviation: f64,
    #[serde(default = "default_width")]
    width: f32,
}
fn default_oriented_particle() -> bool {
    false
}
fn default_particle_alpha_per_part() -> f32 {
    1.0
}
fn default_particle_buffer_size() -> u32 {
    20
}
fn default_particle_end_alpha() -> f32 {
    1.0
}
fn default_particle_fade_out_duration() -> u16 {
    65535
}
fn default_particle_fade_out_threshold() -> f32 {
    1.0
}
fn default_particle_loop_exit_threshold() -> f32 {
    0.0
}
fn default_particle_loop_frame_count() -> u16 {
    1
}
fn default_particle_scale_per_part() -> f32 {
    1.0
}
fn default_particle_start_alpha() -> f32 {
    1.0
}
fn default_particle_start_scale() -> f32 {
    1.0
}
fn default_progress_to_create_smoke() -> f32 {
    0.5
}
fn default_shadow_scale_enabled() -> bool {
    false
}
fn default_target_position_deviation() -> f64 {
    0.0
}
fn default_width() -> f32 {
    0.5
}

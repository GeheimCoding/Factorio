#[derive(Debug, serde::Deserialize)]
pub struct ExplosionPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityPrototype,
    animations: crate::types::AnimationVariations,
    #[serde(default = "default_beam")]
    beam: bool,
    #[serde(default = "default_correct_rotation")]
    correct_rotation: bool,
    #[serde(default = "default_fade_in_duration")]
    fade_in_duration: u8,
    #[serde(default = "default_fade_out_duration")]
    fade_out_duration: u8,
    #[serde(default = "default_height")]
    height: f32,
    light: Option<crate::types::LightDefinition>,
    #[serde(default = "default_light_intensity_factor_final")]
    light_intensity_factor_final: f32,
    #[serde(default = "default_light_intensity_factor_initial")]
    light_intensity_factor_initial: f32,
    #[serde(default = "default_light_intensity_peak_end_progress")]
    light_intensity_peak_end_progress: f32,
    #[serde(default = "default_light_intensity_peak_start_progress")]
    light_intensity_peak_start_progress: f32,
    #[serde(default = "default_light_size_factor_final")]
    light_size_factor_final: f32,
    #[serde(default = "default_light_size_factor_initial")]
    light_size_factor_initial: f32,
    #[serde(default = "default_light_size_peak_end_progress")]
    light_size_peak_end_progress: f32,
    #[serde(default = "default_light_size_peak_start_progress")]
    light_size_peak_start_progress: f32,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_rotate")]
    rotate: bool,
    #[serde(default = "default_scale")]
    scale: f32,
    #[serde(default = "default_scale_animation_speed")]
    scale_animation_speed: bool,
    #[serde(default = "default_scale_deviation")]
    scale_deviation: f32,
    #[serde(default = "default_scale_end")]
    scale_end: f32,
    #[serde(default = "default_scale_in_duration")]
    scale_in_duration: u8,
    #[serde(default = "default_scale_increment_per_tick")]
    scale_increment_per_tick: f32,
    #[serde(default = "default_scale_initial")]
    scale_initial: f32,
    #[serde(default = "default_scale_initial_deviation")]
    scale_initial_deviation: f32,
    #[serde(default = "default_scale_out_duration")]
    scale_out_duration: u8,
    smoke: Option<crate::types::TrivialSmokeID>,
    #[serde(default = "default_smoke_count")]
    smoke_count: u16,
    #[serde(default = "default_smoke_slow_down_factor")]
    smoke_slow_down_factor: f32,
    sound: Option<crate::types::Sound>,
}
fn default_beam() -> bool {
    false
}
fn default_correct_rotation() -> bool {
    false
}
fn default_fade_in_duration() -> u8 {
    0
}
fn default_fade_out_duration() -> u8 {
    0
}
fn default_height() -> f32 {
    1.0
}
fn default_light_intensity_factor_final() -> f32 {
    0.0
}
fn default_light_intensity_factor_initial() -> f32 {
    0.0
}
fn default_light_intensity_peak_end_progress() -> f32 {
    0.9
}
fn default_light_intensity_peak_start_progress() -> f32 {
    0.0
}
fn default_light_size_factor_final() -> f32 {
    0.1
}
fn default_light_size_factor_initial() -> f32 {
    0.1
}
fn default_light_size_peak_end_progress() -> f32 {
    0.5
}
fn default_light_size_peak_start_progress() -> f32 {
    0.1
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Explosion
}
fn default_rotate() -> bool {
    false
}
fn default_scale() -> f32 {
    1.0
}
fn default_scale_animation_speed() -> bool {
    false
}
fn default_scale_deviation() -> f32 {
    0.0
}
fn default_scale_end() -> f32 {
    1.0
}
fn default_scale_in_duration() -> u8 {
    0
}
fn default_scale_increment_per_tick() -> f32 {
    0.0
}
fn default_scale_initial() -> f32 {
    1.0
}
fn default_scale_initial_deviation() -> f32 {
    0.0
}
fn default_scale_out_duration() -> u8 {
    0
}
fn default_smoke_count() -> u16 {
    0
}
fn default_smoke_slow_down_factor() -> f32 {
    0.0
}

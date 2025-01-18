#[derive(serde::Deserialize)]
pub struct ParticlePrototype {
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_draw_shadow_when_on_ground")]
    draw_shadow_when_on_ground: bool,
    ended_in_water_trigger_effect: crate::types::TriggerEffect,
    ended_on_ground_trigger_effect: crate::types::TriggerEffect,
    fade_away_duration: u16,
    life_time: u16,
    #[serde(default = "default_mining_particle_frame_speed")]
    mining_particle_frame_speed: f32,
    #[serde(default = "default_movement_modifier")]
    movement_modifier: f32,
    #[serde(default = "default_movement_modifier_when_on_ground")]
    movement_modifier_when_on_ground: f32,
    pictures: crate::types::AnimationVariations,
    regular_trigger_effect: crate::types::TriggerEffect,
    #[serde(default = "default_regular_trigger_effect_frequency")]
    regular_trigger_effect_frequency: u32,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_render_layer_when_on_ground")]
    render_layer_when_on_ground: crate::types::RenderLayer,
    shadows: crate::types::AnimationVariations,
    #[serde(default = "default_vertical_acceleration")]
    vertical_acceleration: f32,
}
fn default_draw_shadow_when_on_ground() -> bool {
    true
}
fn default_mining_particle_frame_speed() -> f32 {
    0.0
}
fn default_movement_modifier() -> f32 {
    1.0
}
fn default_movement_modifier_when_on_ground() -> f32 {
    0.8
}
fn default_regular_trigger_effect_frequency() -> u32 {
    0
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_render_layer_when_on_ground() -> crate::types::RenderLayer {
    crate::types::RenderLayer::LowerObject
}
fn default_vertical_acceleration() -> f32 {
    -0.0
}

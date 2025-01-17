#[derive(serde::Deserialize)]
pub struct ParticlePrototype {
    base_: crate::prototypes::Prototype,
    draw_shadow_when_on_ground: bool,
    ended_in_water_trigger_effect: crate::types::TriggerEffect,
    ended_on_ground_trigger_effect: crate::types::TriggerEffect,
    fade_away_duration: u16,
    life_time: u16,
    mining_particle_frame_speed: f32,
    movement_modifier: f32,
    movement_modifier_when_on_ground: f32,
    pictures: crate::types::AnimationVariations,
    regular_trigger_effect: crate::types::TriggerEffect,
    regular_trigger_effect_frequency: u32,
    render_layer: crate::types::RenderLayer,
    render_layer_when_on_ground: crate::types::RenderLayer,
    shadows: crate::types::AnimationVariations,
    vertical_acceleration: f32,
}

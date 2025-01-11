pub struct ArtilleryFlarePrototype {
    creation_shift: crate::types::Vector,
    early_death_ticks: u32,
    ended_in_water_trigger_effect: crate::types::TriggerEffect,
    initial_frame_speed: f32,
    initial_height: f32,
    initial_speed: crate::types::Vector,
    initial_vertical_speed: f32,
    life_time: u16,
    map_color: crate::types::Color,
    movement_modifier: f64,
    movement_modifier_when_on_ground: f64,
    pictures: crate::types::AnimationVariations,
    regular_trigger_effect: crate::types::TriggerEffect,
    regular_trigger_effect_frequency: u32,
    render_layer: crate::types::RenderLayer,
    render_layer_when_on_ground: crate::types::RenderLayer,
    selection_priority: u8,
    shadows: crate::types::AnimationVariations,
    shot_category: crate::types::AmmoCategoryID,
    shots_per_flare: u32,
}

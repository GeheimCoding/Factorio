#[derive(Debug, serde::Deserialize)]
pub struct ArtilleryFlarePrototype {
    base_: crate::prototypes::EntityPrototype,
    creation_shift: Option<crate::types::Vector>,
    #[serde(default = "default_early_death_ticks")]
    early_death_ticks: u32,
    ended_in_water_trigger_effect: Option<crate::types::TriggerEffect>,
    #[serde(default = "default_initial_frame_speed")]
    initial_frame_speed: f32,
    #[serde(default = "default_initial_height")]
    initial_height: f32,
    initial_speed: Option<crate::types::Vector>,
    #[serde(default = "default_initial_vertical_speed")]
    initial_vertical_speed: f32,
    life_time: u16,
    map_color: crate::types::Color,
    #[serde(default = "default_movement_modifier")]
    movement_modifier: f64,
    #[serde(default = "default_movement_modifier_when_on_ground")]
    movement_modifier_when_on_ground: f64,
    pictures: Option<crate::types::AnimationVariations>,
    regular_trigger_effect: Option<crate::types::TriggerEffect>,
    #[serde(default = "default_regular_trigger_effect_frequency")]
    regular_trigger_effect_frequency: u32,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_render_layer_when_on_ground")]
    render_layer_when_on_ground: crate::types::RenderLayer,
    #[serde(default = "default_selection_priority")]
    selection_priority: u8,
    shadows: Option<crate::types::AnimationVariations>,
    shot_category: Option<crate::types::AmmoCategoryID>,
    #[serde(default = "default_shots_per_flare")]
    shots_per_flare: u32,
}
fn default_early_death_ticks() -> u32 {
    180
}
fn default_initial_frame_speed() -> f32 {
    1.0
}
fn default_initial_height() -> f32 {
    0.0
}
fn default_initial_vertical_speed() -> f32 {
    0.0
}
fn default_movement_modifier() -> f64 {
    1.0
}
fn default_movement_modifier_when_on_ground() -> f64 {
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
fn default_selection_priority() -> u8 {
    48
}
fn default_shots_per_flare() -> u32 {
    1
}

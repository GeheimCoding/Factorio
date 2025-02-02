#[derive(Debug, serde::Deserialize)]
pub struct MiningDrillPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    // default: All effects are allowed
    allowed_effects: Option<crate::types::EffectTypeLimitation>,
    // default: All module categories are allowed
    allowed_module_categories: Option<Vec<crate::types::ModuleCategoryID>>,
    base_picture: Option<crate::types::Sprite4Way>,
    #[serde(default = "default_base_render_layer")]
    base_render_layer: crate::types::RenderLayer,
    circuit_connector: Option<(
        Box<crate::types::CircuitConnectorDefinition>,
        Box<crate::types::CircuitConnectorDefinition>,
        Box<crate::types::CircuitConnectorDefinition>,
        Box<crate::types::CircuitConnectorDefinition>,
    )>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    drilling_sound: Option<crate::types::InterruptibleSound>,
    #[serde(default = "default_drilling_sound_animation_end_frame")]
    drilling_sound_animation_end_frame: u16,
    #[serde(default = "default_drilling_sound_animation_start_frame")]
    drilling_sound_animation_start_frame: u16,
    #[serde(default = "default_drops_full_belt_stacks")]
    drops_full_belt_stacks: bool,
    effect_receiver: Option<crate::types::EffectReceiver>,
    energy_source: crate::types::EnergySource,
    energy_usage: crate::types::Energy,
    #[serde(default = "default_filter_count")]
    filter_count: u8,
    graphics_set: Option<crate::types::MiningDrillGraphicsSet>,
    input_fluid_box: Option<crate::types::FluidBox>,
    mining_speed: f64,
    module_slots: Option<crate::types::ItemStackIndex>,
    monitor_visualization_tint: Option<crate::types::Color>,
    moving_sound: Option<crate::types::InterruptibleSound>,
    output_fluid_box: Option<crate::types::FluidBox>,
    perceived_performance: Option<crate::types::PerceivedPerformance>,
    radius_visualisation_picture: Option<crate::types::Sprite>,
    resource_categories: Vec<crate::types::ResourceCategoryID>,
    #[serde(default = "default_resource_drain_rate_percent")]
    resource_drain_rate_percent: u8,
    resource_searching_radius: f64,
    #[serde(default = "default_shuffle_resources_to_mine")]
    shuffle_resources_to_mine: bool,
    vector_to_place_result: crate::types::Vector,
    wet_mining_graphics_set: Option<crate::types::MiningDrillGraphicsSet>,
}
fn default_base_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::LowerObject
}
fn default_circuit_wire_max_distance() -> f64 {
    0.0
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
fn default_drilling_sound_animation_end_frame() -> u16 {
    0
}
fn default_drilling_sound_animation_start_frame() -> u16 {
    0
}
fn default_drops_full_belt_stacks() -> bool {
    false
}
fn default_filter_count() -> u8 {
    0
}
fn default_resource_drain_rate_percent() -> u8 {
    100
}
fn default_shuffle_resources_to_mine() -> bool {
    false
}

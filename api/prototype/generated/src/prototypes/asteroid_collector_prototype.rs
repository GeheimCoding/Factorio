#[derive(serde::Deserialize)]
pub struct AsteroidCollectorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_arm_angular_speed_cap_base")]
    arm_angular_speed_cap_base: f32,
    #[serde(default = "default_arm_angular_speed_cap_quality_scaling")]
    arm_angular_speed_cap_quality_scaling: f32,
    // default: {{1, 1, 1}}
    arm_color_gradient: Option<Vec<crate::types::Color>>,
    #[serde(default = "default_arm_count_base")]
    arm_count_base: u32,
    #[serde(default = "default_arm_count_quality_scaling")]
    arm_count_quality_scaling: u32,
    arm_energy_usage: crate::types::Energy,
    arm_extend_sound: Option<crate::types::Sound>,
    #[serde(default = "default_arm_inventory_size")]
    arm_inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_arm_inventory_size_quality_increase")]
    arm_inventory_size_quality_increase: crate::types::ItemStackIndex,
    arm_retract_sound: Option<crate::types::Sound>,
    arm_slow_energy_usage: crate::types::Energy,
    #[serde(default = "default_arm_speed_base")]
    arm_speed_base: f32,
    #[serde(default = "default_arm_speed_quality_scaling")]
    arm_speed_quality_scaling: f32,
    circuit_connector: Option<(
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    )>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_collection_box_offset")]
    collection_box_offset: f32,
    collection_radius: f64,
    #[serde(default = "default_deposit_radius")]
    deposit_radius: f32,
    deposit_sound: Option<crate::types::Sound>,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    energy_source: AsteroidCollectorPrototypeEnergySource,
    #[serde(default = "default_energy_usage_quality_scaling")]
    energy_usage_quality_scaling: f32,
    graphics_set: AsteroidCollectorGraphicsSet,
    #[serde(default = "default_head_collection_radius")]
    head_collection_radius: f32,
    #[serde(default = "default_held_items_display_count")]
    held_items_display_count: u8,
    #[serde(default = "default_held_items_offset")]
    held_items_offset: f32,
    #[serde(default = "default_held_items_spread")]
    held_items_spread: f32,
    #[serde(default = "default_inventory_size")]
    inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_inventory_size_quality_increase")]
    inventory_size_quality_increase: crate::types::ItemStackIndex,
    #[serde(default = "default_minimal_arm_swing_segment_retraction")]
    minimal_arm_swing_segment_retraction: u32,
    munch_sound: Option<crate::types::Sound>,
    passive_energy_usage: crate::types::Energy,
    radius_visualisation_picture: Option<crate::types::Sprite>,
    #[serde(default = "default_tether_size")]
    tether_size: f32,
    #[serde(default = "default_unpowered_arm_speed_scale")]
    unpowered_arm_speed_scale: f32,
}
fn default_arm_angular_speed_cap_base() -> f32 {
    0.1
}
fn default_arm_angular_speed_cap_quality_scaling() -> f32 {
    0.1
}
fn default_arm_count_base() -> u32 {
    3
}
fn default_arm_count_quality_scaling() -> u32 {
    1
}
fn default_arm_inventory_size() -> crate::types::ItemStackIndex {
    5
}
fn default_arm_inventory_size_quality_increase() -> crate::types::ItemStackIndex {
    0
}
fn default_arm_speed_base() -> f32 {
    0.1
}
fn default_arm_speed_quality_scaling() -> f32 {
    0.1
}
fn default_circuit_wire_max_distance() -> f64 {
    0.0
}
fn default_collection_box_offset() -> f32 {
    0.0
}
fn default_deposit_radius() -> f32 {
    1.5
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
#[derive(serde::Deserialize)]
pub enum AsteroidCollectorPrototypeEnergySource {
    #[serde(untagged)]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
fn default_energy_usage_quality_scaling() -> f32 {
    0.1
}
#[derive(serde::Deserialize)]
pub struct AsteroidCollectorGraphicsSet {
    animation: Option<crate::types::Animation4Way>,
    arm_head_animation: Option<crate::types::RotatedAnimation>,
    arm_head_top_animation: Option<crate::types::RotatedAnimation>,
    arm_link: Option<crate::types::RotatedSprite>,
    below_arm_pictures: Option<crate::types::RotatedSprite>,
    below_ground_pictures: Option<crate::types::RotatedSprite>,
    status_lamp_picture_full: Option<crate::types::RotatedSprite>,
    status_lamp_picture_off: Option<crate::types::RotatedSprite>,
    status_lamp_picture_on: Option<crate::types::RotatedSprite>,
}
fn default_head_collection_radius() -> f32 {
    0.6
}
fn default_held_items_display_count() -> u8 {
    5
}
fn default_held_items_offset() -> f32 {
    0.1
}
fn default_held_items_spread() -> f32 {
    0.1
}
fn default_inventory_size() -> crate::types::ItemStackIndex {
    39
}
fn default_inventory_size_quality_increase() -> crate::types::ItemStackIndex {
    5
}
fn default_minimal_arm_swing_segment_retraction() -> u32 {
    6
}
fn default_tether_size() -> f32 {
    0.4
}
fn default_unpowered_arm_speed_scale() -> f32 {
    0.3
}

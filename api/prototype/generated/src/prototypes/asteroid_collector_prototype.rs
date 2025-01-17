#[derive(serde::Deserialize)]
pub struct AsteroidCollectorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    arm_angular_speed_cap_base: f32,
    arm_angular_speed_cap_quality_scaling: f32,
    arm_color_gradient: Vec<crate::types::Color>,
    arm_count_base: u32,
    arm_count_quality_scaling: u32,
    arm_energy_usage: crate::types::Energy,
    arm_extend_sound: crate::types::Sound,
    arm_inventory_size: crate::types::ItemStackIndex,
    arm_inventory_size_quality_increase: crate::types::ItemStackIndex,
    arm_retract_sound: crate::types::Sound,
    arm_slow_energy_usage: crate::types::Energy,
    arm_speed_base: f32,
    arm_speed_quality_scaling: f32,
    circuit_connector: (
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    ),
    circuit_wire_max_distance: f64,
    collection_box_offset: f32,
    collection_radius: f64,
    deposit_radius: f32,
    deposit_sound: crate::types::Sound,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    energy_source: AsteroidCollectorPrototypeEnergySource,
    energy_usage_quality_scaling: f32,
    graphics_set: AsteroidCollectorGraphicsSet,
    head_collection_radius: f32,
    held_items_display_count: u8,
    held_items_offset: f32,
    held_items_spread: f32,
    inventory_size: crate::types::ItemStackIndex,
    inventory_size_quality_increase: crate::types::ItemStackIndex,
    minimal_arm_swing_segment_retraction: u32,
    munch_sound: crate::types::Sound,
    passive_energy_usage: crate::types::Energy,
    radius_visualisation_picture: crate::types::Sprite,
    tether_size: f32,
    unpowered_arm_speed_scale: f32,
}
#[derive(serde::Deserialize)]
pub enum AsteroidCollectorPrototypeEnergySource {
    #[serde(untagged)]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
#[derive(serde::Deserialize)]
pub struct AsteroidCollectorGraphicsSet {
    animation: crate::types::Animation4Way,
    arm_head_animation: crate::types::RotatedAnimation,
    arm_head_top_animation: crate::types::RotatedAnimation,
    arm_link: crate::types::RotatedSprite,
    below_arm_pictures: crate::types::RotatedSprite,
    below_ground_pictures: crate::types::RotatedSprite,
    status_lamp_picture_full: crate::types::RotatedSprite,
    status_lamp_picture_off: crate::types::RotatedSprite,
    status_lamp_picture_on: crate::types::RotatedSprite,
}

#[derive(serde::Deserialize)]
pub struct CombinatorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    active_energy_usage: crate::types::Energy,
    #[serde(default = "default_activity_led_hold_time")]
    activity_led_hold_time: u8,
    activity_led_light: crate::types::LightDefinition,
    activity_led_light_offsets: (
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
    ),
    activity_led_sprites: crate::types::Sprite4Way,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    emissions_per_second: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
    energy_source: CombinatorPrototypeEnergySource,
    frozen_patch: crate::types::Sprite4Way,
    input_connection_bounding_box: crate::types::BoundingBox,
    input_connection_points: (
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
    ),
    output_connection_bounding_box: crate::types::BoundingBox,
    output_connection_points: (
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
    ),
    screen_light: crate::types::LightDefinition,
    screen_light_offsets: (
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
    ),
    sprites: crate::types::Sprite4Way,
}
fn default_activity_led_hold_time() -> u8 {
    5
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
#[derive(serde::Deserialize)]
pub enum CombinatorPrototypeEnergySource {
    #[serde(untagged)]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}

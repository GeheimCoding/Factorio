#[derive(Debug, serde::Deserialize)]
pub struct CombinatorPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    active_energy_usage: crate::types::Energy,
    #[serde(default = "default_activity_led_hold_time")]
    activity_led_hold_time: u8,
    activity_led_light: Option<crate::types::LightDefinition>,
    activity_led_light_offsets: (
        Box<crate::types::Vector>,
        Box<crate::types::Vector>,
        Box<crate::types::Vector>,
        Box<crate::types::Vector>,
    ),
    activity_led_sprites: Option<crate::types::Sprite4Way>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    emissions_per_second: Option<std::collections::HashMap<crate::types::AirbornePollutantID, f64>>,
    energy_source: CombinatorPrototypeEnergySource,
    frozen_patch: Option<crate::types::Sprite4Way>,
    input_connection_bounding_box: crate::types::BoundingBox,
    input_connection_points: (
        Box<crate::types::WireConnectionPoint>,
        Box<crate::types::WireConnectionPoint>,
        Box<crate::types::WireConnectionPoint>,
        Box<crate::types::WireConnectionPoint>,
    ),
    output_connection_bounding_box: crate::types::BoundingBox,
    output_connection_points: (
        Box<crate::types::WireConnectionPoint>,
        Box<crate::types::WireConnectionPoint>,
        Box<crate::types::WireConnectionPoint>,
        Box<crate::types::WireConnectionPoint>,
    ),
    screen_light: Option<crate::types::LightDefinition>,
    screen_light_offsets: (
        Box<crate::types::Vector>,
        Box<crate::types::Vector>,
        Box<crate::types::Vector>,
        Box<crate::types::Vector>,
    ),
    sprites: Option<crate::types::Sprite4Way>,
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
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum CombinatorPrototypeEnergySource {
    #[serde(rename = "electric")]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(rename = "void")]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}

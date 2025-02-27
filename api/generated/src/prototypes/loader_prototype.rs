#[derive(Debug, serde::Deserialize)]
pub struct LoaderPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::TransportBeltConnectablePrototype,
    #[serde(default = "default_allow_container_interaction")]
    allow_container_interaction: bool,
    #[serde(default = "default_allow_rail_interaction")]
    allow_rail_interaction: bool,
    #[serde(default = "default_belt_length")]
    belt_length: f64,
    circuit_connector: Option<crate::vec::Vec<crate::types::CircuitConnectorDefinition>>,
    #[serde(default = "default_circuit_connector_layer")]
    circuit_connector_layer: crate::types::RenderLayer,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_container_distance")]
    container_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    #[serde(default = "default_energy_per_item")]
    energy_per_item: crate::types::Energy,
    energy_source: Option<LoaderPrototypeEnergySource>,
    filter_count: u8,
    #[serde(default = "default_max_belt_stack_size")]
    max_belt_stack_size: u8,
    #[serde(default = "default_per_lane_filters")]
    per_lane_filters: bool,
    structure: Option<crate::types::LoaderStructure>,
    #[serde(default = "default_structure_render_layer")]
    structure_render_layer: crate::types::RenderLayer,
}
fn default_allow_container_interaction() -> bool {
    true
}
fn default_allow_rail_interaction() -> bool {
    true
}
fn default_belt_length() -> f64 {
    0.5
}
fn default_circuit_connector_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_circuit_wire_max_distance() -> f64 {
    0.0
}
fn default_container_distance() -> f64 {
    1.5
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
fn default_energy_per_item() -> crate::types::Energy {
    String::from("0")
}
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum LoaderPrototypeEnergySource {
    #[serde(rename = "electric")]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(rename = "heat")]
    HeatEnergySource(Box<crate::types::HeatEnergySource>),
    #[serde(rename = "fluid")]
    FluidEnergySource(Box<crate::types::FluidEnergySource>),
    #[serde(rename = "void")]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
fn default_max_belt_stack_size() -> u8 {
    1
}
fn default_per_lane_filters() -> bool {
    false
}
fn default_structure_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}

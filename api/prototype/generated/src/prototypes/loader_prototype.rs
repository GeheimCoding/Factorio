pub struct LoaderPrototype {
    base_: crate::prototypes::TransportBeltConnectablePrototype,
    allow_container_interaction: bool,
    allow_rail_interaction: bool,
    belt_length: f64,
    circuit_connector: Vec<crate::types::CircuitConnectorDefinition>,
    circuit_connector_layer: crate::types::RenderLayer,
    circuit_wire_max_distance: f64,
    container_distance: f64,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    energy_per_item: crate::types::Energy,
    energy_source: LoaderPrototypeEnergySource,
    filter_count: u8,
    max_belt_stack_size: u8,
    structure: crate::types::LoaderStructure,
    structure_render_layer: crate::types::RenderLayer,
}
#[derive(serde::Deserialize)]
pub enum LoaderPrototypeEnergySource {
    #[serde(untagged)]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(untagged)]
    HeatEnergySource(Box<crate::types::HeatEnergySource>),
    #[serde(untagged)]
    FluidEnergySource(Box<crate::types::FluidEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}

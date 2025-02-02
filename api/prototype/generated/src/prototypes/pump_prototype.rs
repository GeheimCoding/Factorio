#[derive(Debug, serde::Deserialize)]
pub struct PumpPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    animations: Option<crate::types::Animation4Way>,
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
    energy_source: crate::types::EnergySource,
    energy_usage: crate::types::Energy,
    fluid_animation: Option<crate::types::Animation4Way>,
    fluid_box: crate::types::FluidBox,
    // default: 2 / 32.0
    fluid_wagon_connector_alignment_tolerance: Option<f64>,
    #[serde(default = "default_fluid_wagon_connector_frame_count")]
    fluid_wagon_connector_frame_count: u8,
    fluid_wagon_connector_graphics: Option<FluidWagonConnectorGraphics>,
    // default: 1 / 64.0
    fluid_wagon_connector_speed: Option<f64>,
    frozen_patch: Option<crate::types::Sprite4Way>,
    glass_pictures: Option<crate::types::Sprite4Way>,
    pumping_speed: crate::types::FluidAmount,
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
fn default_fluid_wagon_connector_frame_count() -> u8 {
    1
}
#[derive(Debug, serde::Deserialize)]
pub struct FluidWagonConnectorGraphics {
    load_animations: crate::types::PumpConnectorGraphics,
    unload_animations: crate::types::PumpConnectorGraphics,
}

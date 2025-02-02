#[derive(Debug, serde::Deserialize)]
pub struct OffshorePumpPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_always_draw_fluid")]
    always_draw_fluid: bool,
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
    fluid_box: crate::types::FluidBox,
    fluid_source_offset: crate::types::Vector,
    graphics_set: Option<crate::types::OffshorePumpGraphicsSet>,
    perceived_performance: Option<crate::types::PerceivedPerformance>,
    pumping_speed: crate::types::FluidAmount,
    #[serde(default = "default_remove_on_tile_collision")]
    remove_on_tile_collision: bool,
}
fn default_always_draw_fluid() -> bool {
    true
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
fn default_remove_on_tile_collision() -> bool {
    false
}

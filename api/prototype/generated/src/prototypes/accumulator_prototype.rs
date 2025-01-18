#[derive(serde::Deserialize)]
pub struct AccumulatorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    chargable_graphics: crate::types::ChargableGraphics,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    default_output_signal: crate::types::SignalIDConnector,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    energy_source: crate::types::ElectricEnergySource,
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

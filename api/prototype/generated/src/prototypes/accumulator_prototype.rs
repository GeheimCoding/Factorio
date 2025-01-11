pub struct AccumulatorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    chargable_graphics: crate::types::ChargableGraphics,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    default_output_signal: crate::types::SignalIDConnector,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    energy_source: crate::types::ElectricEnergySource,
}

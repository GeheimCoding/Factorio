#[derive(serde::Deserialize)]
pub struct OffshorePumpPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    always_draw_fluid: bool,
    circuit_connector: (
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    ),
    circuit_wire_max_distance: f64,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    energy_source: crate::types::EnergySource,
    energy_usage: crate::types::Energy,
    fluid_box: crate::types::FluidBox,
    fluid_source_offset: crate::types::Vector,
    graphics_set: crate::types::OffshorePumpGraphicsSet,
    perceived_performance: crate::types::PerceivedPerformance,
    pumping_speed: crate::types::FluidAmount,
    remove_on_tile_collision: bool,
}

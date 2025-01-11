pub struct PumpPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    animations: crate::types::Animation4Way,
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
    fluid_animation: crate::types::Animation4Way,
    fluid_box: crate::types::FluidBox,
    fluid_wagon_connector_alignment_tolerance: f64,
    fluid_wagon_connector_frame_count: u8,
    fluid_wagon_connector_graphics: FluidWagonConnectorGraphics,
    fluid_wagon_connector_speed: f64,
    frozen_patch: crate::types::Sprite4Way,
    glass_pictures: crate::types::Sprite4Way,
    pumping_speed: crate::types::FluidAmount,
}
pub struct FluidWagonConnectorGraphics {
    load_animations: crate::types::PumpConnectorGraphics,
    unload_animations: crate::types::PumpConnectorGraphics,
}

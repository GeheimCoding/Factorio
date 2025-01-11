pub struct StorageTankPrototype {
    circuit_connector: (
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    ),
    circuit_wire_max_distance: f64,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    flow_length_in_ticks: u32,
    fluid_box: crate::types::FluidBox,
    pictures: StorageTankPictures,
    show_fluid_icon: bool,
    two_direction_only: bool,
    window_bounding_box: crate::types::BoundingBox,
}
pub struct StorageTankPictures {
    flow_sprite: crate::types::Sprite,
    fluid_background: crate::types::Sprite,
    frozen_patch: crate::types::Sprite4Way,
    gas_flow: crate::types::Animation,
    picture: crate::types::Sprite4Way,
    window_background: crate::types::Sprite,
}

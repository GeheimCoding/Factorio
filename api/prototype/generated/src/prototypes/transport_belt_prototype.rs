pub struct TransportBeltPrototype {
    belt_animation_set: crate::types::TransportBeltAnimationSetWithCorners,
    circuit_connector: Vec<crate::types::CircuitConnectorDefinition>,
    circuit_wire_max_distance: f64,
    connector_frame_sprites: crate::types::TransportBeltConnectorFrame,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    related_underground_belt: crate::types::EntityID,
}

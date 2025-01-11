pub struct DisplayPanelPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    background_color: crate::types::Color,
    circuit_connector: (
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    ),
    circuit_wire_max_distance: f64,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    max_text_width: u32,
    sprites: crate::types::Sprite4Way,
    text_color: crate::types::Color,
    text_shift: crate::types::Vector,
}

#[derive(Debug, serde::Deserialize)]
pub struct DisplayPanelPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    background_color: Option<crate::types::Color>,
    circuit_connector: Option<(
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    )>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    #[serde(default = "default_max_text_width")]
    max_text_width: u32,
    sprites: Option<crate::types::Sprite4Way>,
    text_color: Option<crate::types::Color>,
    text_shift: Option<crate::types::Vector>,
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
fn default_max_text_width() -> u32 {
    400
}

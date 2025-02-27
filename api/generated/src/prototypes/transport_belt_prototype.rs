#[derive(Debug, serde::Deserialize)]
pub struct TransportBeltPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::TransportBeltConnectablePrototype,
    belt_animation_set: Option<crate::types::TransportBeltAnimationSetWithCorners>,
    circuit_connector: Option<crate::vec::Vec<crate::types::CircuitConnectorDefinition>>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    connector_frame_sprites: Option<crate::types::TransportBeltConnectorFrame>,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    related_underground_belt: Option<crate::types::EntityID>,
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

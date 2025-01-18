#[derive(serde::Deserialize)]
pub struct RailSignalBasePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    // default: `{{-0.2, -0.2}, {0.2, 0.2}}`
    collision_box: crate::types::BoundingBox,
    default_blue_output_signal: crate::types::SignalIDConnector,
    default_green_output_signal: crate::types::SignalIDConnector,
    default_orange_output_signal: crate::types::SignalIDConnector,
    default_red_output_signal: crate::types::SignalIDConnector,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    elevated_collision_mask: crate::types::CollisionMaskConnector,
    elevated_picture_set: crate::types::RailSignalPictureSet,
    #[serde(default = "default_elevated_selection_priority")]
    elevated_selection_priority: u8,
    flags: crate::types::EntityPrototypeFlags,
    ground_picture_set: crate::types::RailSignalPictureSet,
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
fn default_elevated_selection_priority() -> u8 {
    55
}

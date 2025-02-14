#[derive(Debug, serde::Deserialize)]
pub struct RailSignalBasePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    // default: `{{-0.2, -0.2}, {0.2, 0.2}}`
    collision_box: Option<crate::types::BoundingBox>,
    default_blue_output_signal: Option<crate::types::SignalIDConnector>,
    default_green_output_signal: Option<crate::types::SignalIDConnector>,
    default_orange_output_signal: Option<crate::types::SignalIDConnector>,
    default_red_output_signal: Option<crate::types::SignalIDConnector>,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    elevated_collision_mask: Option<crate::types::CollisionMaskConnector>,
    elevated_picture_set: crate::types::RailSignalPictureSet,
    #[serde(default = "default_elevated_selection_priority")]
    elevated_selection_priority: u8,
    flags: Option<crate::types::EntityPrototypeFlags>,
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

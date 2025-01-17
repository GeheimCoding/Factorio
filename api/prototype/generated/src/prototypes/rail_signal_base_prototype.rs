#[derive(serde::Deserialize)]
pub struct RailSignalBasePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_wire_max_distance: f64,
    collision_box: crate::types::BoundingBox,
    default_blue_output_signal: crate::types::SignalIDConnector,
    default_green_output_signal: crate::types::SignalIDConnector,
    default_orange_output_signal: crate::types::SignalIDConnector,
    default_red_output_signal: crate::types::SignalIDConnector,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    elevated_collision_mask: crate::types::CollisionMaskConnector,
    elevated_picture_set: crate::types::RailSignalPictureSet,
    elevated_selection_priority: u8,
    flags: crate::types::EntityPrototypeFlags,
    ground_picture_set: crate::types::RailSignalPictureSet,
}

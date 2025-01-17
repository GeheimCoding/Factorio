#[derive(serde::Deserialize)]
pub struct TrainStopPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    animation_ticks_per_frame: u32,
    animations: crate::types::Animation4Way,
    build_grid_size: String,
    chart_name: bool,
    circuit_connector: (
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    ),
    circuit_wire_max_distance: f64,
    color: crate::types::Color,
    default_priority_signal: crate::types::SignalIDConnector,
    default_train_stopped_signal: crate::types::SignalIDConnector,
    default_trains_count_signal: crate::types::SignalIDConnector,
    default_trains_limit_signal: crate::types::SignalIDConnector,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    drawing_boxes: TrainStopDrawingBoxes,
    light1: crate::types::TrainStopLight,
    light2: crate::types::TrainStopLight,
    rail_overlay_animations: crate::types::Animation4Way,
    top_animations: crate::types::Animation4Way,
}
#[derive(serde::Deserialize)]
pub struct TrainStopDrawingBoxes {
    east: crate::types::BoundingBox,
    north: crate::types::BoundingBox,
    south: crate::types::BoundingBox,
    west: crate::types::BoundingBox,
}

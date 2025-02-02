#[derive(Debug, serde::Deserialize)]
pub struct TrainStopPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    animation_ticks_per_frame: u32,
    animations: Option<crate::types::Animation4Way>,
    #[serde(default = "default_build_grid_size")]
    build_grid_size: f64,
    #[serde(default = "default_chart_name")]
    chart_name: bool,
    circuit_connector: Option<(
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    )>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    color: Option<crate::types::Color>,
    default_priority_signal: Option<crate::types::SignalIDConnector>,
    default_train_stopped_signal: Option<crate::types::SignalIDConnector>,
    default_trains_count_signal: Option<crate::types::SignalIDConnector>,
    default_trains_limit_signal: Option<crate::types::SignalIDConnector>,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    drawing_boxes: Option<TrainStopDrawingBoxes>,
    #[serde(rename = "light1")]
    light_1: Option<crate::types::TrainStopLight>,
    #[serde(rename = "light2")]
    light_2: Option<crate::types::TrainStopLight>,
    rail_overlay_animations: Option<crate::types::Animation4Way>,
    top_animations: Option<crate::types::Animation4Way>,
}
fn default_build_grid_size() -> f64 {
    2.0
}
fn default_chart_name() -> bool {
    true
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
#[derive(Debug, serde::Deserialize)]
pub struct TrainStopDrawingBoxes {
    east: crate::types::BoundingBox,
    north: crate::types::BoundingBox,
    south: crate::types::BoundingBox,
    west: crate::types::BoundingBox,
}

#[derive(serde::Deserialize)]
pub struct MiningDrillGraphicsSet {
    base_: crate::types::WorkingVisualisations,
    #[serde(default = "default_animation_progress")]
    animation_progress: f32,
    #[serde(default = "default_circuit_connector_layer")]
    circuit_connector_layer: MiningDrillGraphicsSetCircuitConnectorLayer,
    #[serde(default = "default_circuit_connector_secondary_draw_order")]
    circuit_connector_secondary_draw_order:
        MiningDrillGraphicsSetCircuitConnectorSecondaryDrawOrder,
    #[serde(default = "default_drilling_vertical_movement_duration")]
    drilling_vertical_movement_duration: u16,
    frozen_patch: Option<crate::types::Sprite4Way>,
    #[serde(default = "default_reset_animation_when_frozen")]
    reset_animation_when_frozen: bool,
}
fn default_animation_progress() -> f32 {
    1.0
}
#[derive(serde::Deserialize)]
pub enum MiningDrillGraphicsSetCircuitConnectorLayer {
    #[serde(untagged)]
    RenderLayer(crate::types::RenderLayer),
    #[serde(untagged)]
    CircuitConnectorLayer(Box<crate::types::CircuitConnectorLayer>),
}
fn default_circuit_connector_layer() -> MiningDrillGraphicsSetCircuitConnectorLayer {
    MiningDrillGraphicsSetCircuitConnectorLayer::RenderLayer(crate::types::RenderLayer::Object)
}
#[derive(serde::Deserialize)]
pub enum MiningDrillGraphicsSetCircuitConnectorSecondaryDrawOrder {
    #[serde(untagged)]
    I8(i8),
    #[serde(untagged)]
    CircuitConnectorSecondaryDrawOrder(Box<crate::types::CircuitConnectorSecondaryDrawOrder>),
}
fn default_circuit_connector_secondary_draw_order(
) -> MiningDrillGraphicsSetCircuitConnectorSecondaryDrawOrder {
    MiningDrillGraphicsSetCircuitConnectorSecondaryDrawOrder::I8(100)
}
fn default_drilling_vertical_movement_duration() -> u16 {
    0
}
fn default_reset_animation_when_frozen() -> bool {
    false
}

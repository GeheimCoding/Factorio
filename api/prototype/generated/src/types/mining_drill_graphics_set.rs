pub struct MiningDrillGraphicsSet {
    base_: crate::types::WorkingVisualisations,
    animation_progress: f32,
    circuit_connector_layer: MiningDrillGraphicsSetCircuitConnectorLayer,
    circuit_connector_secondary_draw_order:
        MiningDrillGraphicsSetCircuitConnectorSecondaryDrawOrder,
    drilling_vertical_movement_duration: u16,
    frozen_patch: crate::types::Sprite4Way,
    reset_animation_when_frozen: bool,
}
#[derive(serde::Deserialize)]
pub enum MiningDrillGraphicsSetCircuitConnectorLayer {
    #[serde(untagged)]
    RenderLayer(crate::types::RenderLayer),
    #[serde(untagged)]
    CircuitConnectorLayer(Box<crate::types::CircuitConnectorLayer>),
}
#[derive(serde::Deserialize)]
pub enum MiningDrillGraphicsSetCircuitConnectorSecondaryDrawOrder {
    #[serde(untagged)]
    I8(i8),
    #[serde(untagged)]
    CircuitConnectorSecondaryDrawOrder(Box<crate::types::CircuitConnectorSecondaryDrawOrder>),
}

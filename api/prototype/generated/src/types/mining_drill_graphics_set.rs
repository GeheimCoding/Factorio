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
pub enum MiningDrillGraphicsSetCircuitConnectorLayer {
    RenderLayer(crate::types::RenderLayer),
    CircuitConnectorLayer(Box<crate::types::CircuitConnectorLayer>),
}
pub enum MiningDrillGraphicsSetCircuitConnectorSecondaryDrawOrder {
    I8(i8),
    CircuitConnectorSecondaryDrawOrder(Box<crate::types::CircuitConnectorSecondaryDrawOrder>),
}

pub struct MiningDrillGraphicsSet {
    animation_progress: f32,
    circuit_connector_layer: MiningDrillGraphicsSetCircuitConnectorLayer,
    circuit_connector_secondary_draw_order:
        MiningDrillGraphicsSetCircuitConnectorSecondaryDrawOrder,
    drilling_vertical_movement_duration: u16,
    frozen_patch: Sprite4Way,
    reset_animation_when_frozen: bool,
}
pub enum MiningDrillGraphicsSetCircuitConnectorLayer {
    RenderLayer(RenderLayer),
    CircuitConnectorLayer(CircuitConnectorLayer),
}
pub enum MiningDrillGraphicsSetCircuitConnectorSecondaryDrawOrder {
    I8(i8),
    CircuitConnectorSecondaryDrawOrder(CircuitConnectorSecondaryDrawOrder),
}

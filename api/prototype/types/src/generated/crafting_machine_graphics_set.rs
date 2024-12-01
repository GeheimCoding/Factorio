pub struct CraftingMachineGraphicsSet {
    animation_progress: f32,
    circuit_connector_layer: CraftingMachineGraphicsSetCircuitConnectorLayer,
    circuit_connector_secondary_draw_order:
        CraftingMachineGraphicsSetCircuitConnectorSecondaryDrawOrder,
    frozen_patch: Sprite4Way,
    reset_animation_when_frozen: bool,
}
pub enum CraftingMachineGraphicsSetCircuitConnectorLayer {
    RenderLayer(RenderLayer),
    CircuitConnectorLayer(CircuitConnectorLayer),
}
pub enum CraftingMachineGraphicsSetCircuitConnectorSecondaryDrawOrder {
    I8(i8),
    CircuitConnectorSecondaryDrawOrder(CircuitConnectorSecondaryDrawOrder),
}

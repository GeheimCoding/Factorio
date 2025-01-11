pub struct CraftingMachineGraphicsSet {
    base_: crate::types::WorkingVisualisations,
    animation_progress: f32,
    circuit_connector_layer: CraftingMachineGraphicsSetCircuitConnectorLayer,
    circuit_connector_secondary_draw_order:
        CraftingMachineGraphicsSetCircuitConnectorSecondaryDrawOrder,
    frozen_patch: crate::types::Sprite4Way,
    reset_animation_when_frozen: bool,
}
pub enum CraftingMachineGraphicsSetCircuitConnectorLayer {
    RenderLayer(crate::types::RenderLayer),
    CircuitConnectorLayer(Box<crate::types::CircuitConnectorLayer>),
}
pub enum CraftingMachineGraphicsSetCircuitConnectorSecondaryDrawOrder {
    I8(i8),
    CircuitConnectorSecondaryDrawOrder(Box<crate::types::CircuitConnectorSecondaryDrawOrder>),
}

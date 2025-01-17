#[derive(serde::Deserialize)]
pub struct CraftingMachineGraphicsSet {
    base_: crate::types::WorkingVisualisations,
    animation_progress: f32,
    circuit_connector_layer: CraftingMachineGraphicsSetCircuitConnectorLayer,
    circuit_connector_secondary_draw_order:
        CraftingMachineGraphicsSetCircuitConnectorSecondaryDrawOrder,
    frozen_patch: crate::types::Sprite4Way,
    reset_animation_when_frozen: bool,
}
#[derive(serde::Deserialize)]
pub enum CraftingMachineGraphicsSetCircuitConnectorLayer {
    #[serde(untagged)]
    RenderLayer(crate::types::RenderLayer),
    #[serde(untagged)]
    CircuitConnectorLayer(Box<crate::types::CircuitConnectorLayer>),
}
#[derive(serde::Deserialize)]
pub enum CraftingMachineGraphicsSetCircuitConnectorSecondaryDrawOrder {
    #[serde(untagged)]
    I8(i8),
    #[serde(untagged)]
    CircuitConnectorSecondaryDrawOrder(Box<crate::types::CircuitConnectorSecondaryDrawOrder>),
}

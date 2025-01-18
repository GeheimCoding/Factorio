#[derive(serde::Deserialize)]
pub struct CraftingMachineGraphicsSet {
    base_: crate::types::WorkingVisualisations,
    #[serde(default = "default_animation_progress")]
    animation_progress: f32,
    #[serde(default = "default_circuit_connector_layer")]
    circuit_connector_layer: CraftingMachineGraphicsSetCircuitConnectorLayer,
    #[serde(default = "default_circuit_connector_secondary_draw_order")]
    circuit_connector_secondary_draw_order:
        CraftingMachineGraphicsSetCircuitConnectorSecondaryDrawOrder,
    frozen_patch: crate::types::Sprite4Way,
    #[serde(default = "default_reset_animation_when_frozen")]
    reset_animation_when_frozen: bool,
}
fn default_animation_progress() -> f32 {
    0.5
}
#[derive(serde::Deserialize)]
pub enum CraftingMachineGraphicsSetCircuitConnectorLayer {
    #[serde(untagged)]
    RenderLayer(crate::types::RenderLayer),
    #[serde(untagged)]
    CircuitConnectorLayer(Box<crate::types::CircuitConnectorLayer>),
}
fn default_circuit_connector_layer() -> CraftingMachineGraphicsSetCircuitConnectorLayer {
    CraftingMachineGraphicsSetCircuitConnectorLayer::RenderLayer(crate::types::RenderLayer::Object)
}
#[derive(serde::Deserialize)]
pub enum CraftingMachineGraphicsSetCircuitConnectorSecondaryDrawOrder {
    #[serde(untagged)]
    I8(i8),
    #[serde(untagged)]
    CircuitConnectorSecondaryDrawOrder(Box<crate::types::CircuitConnectorSecondaryDrawOrder>),
}
fn default_circuit_connector_secondary_draw_order(
) -> CraftingMachineGraphicsSetCircuitConnectorSecondaryDrawOrder {
    CraftingMachineGraphicsSetCircuitConnectorSecondaryDrawOrder::I8(100)
}
fn default_reset_animation_when_frozen() -> bool {
    false
}

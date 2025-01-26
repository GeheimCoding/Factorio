#[derive(Debug, serde::Deserialize)]
pub struct ContainerPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_connector: Option<crate::types::CircuitConnectorDefinition>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_default_status")]
    default_status: crate::types::EntityStatus,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_inventory_type")]
    inventory_type: ContainerPrototypeInventoryType,
    picture: Option<crate::types::Sprite>,
}
fn default_circuit_wire_max_distance() -> f64 {
    0.0
}
fn default_default_status() -> crate::types::EntityStatus {
    crate::types::EntityStatus::Normal
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
#[derive(Debug, serde::Deserialize)]
pub enum ContainerPrototypeInventoryType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "with_bar")]
    WithBar,
    #[serde(rename = "with_filters_and_bar")]
    WithFiltersAndBar,
}
fn default_inventory_type() -> ContainerPrototypeInventoryType {
    ContainerPrototypeInventoryType::WithBar
}

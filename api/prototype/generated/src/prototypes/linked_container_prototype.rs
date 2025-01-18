#[derive(serde::Deserialize)]
pub struct LinkedContainerPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    #[serde(default = "default_gui_mode")]
    gui_mode: LinkedContainerPrototypeGuiMode,
    inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_inventory_type")]
    inventory_type: LinkedContainerPrototypeInventoryType,
    picture: crate::types::Sprite,
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
#[derive(serde::Deserialize)]
pub enum LinkedContainerPrototypeGuiMode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "admins")]
    Admins,
}
fn default_gui_mode() -> LinkedContainerPrototypeGuiMode {
    LinkedContainerPrototypeGuiMode::All
}
#[derive(serde::Deserialize)]
pub enum LinkedContainerPrototypeInventoryType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "with_bar")]
    WithBar,
    #[serde(rename = "with_filters_and_bar")]
    WithFiltersAndBar,
}
fn default_inventory_type() -> LinkedContainerPrototypeInventoryType {
    LinkedContainerPrototypeInventoryType::WithBar
}

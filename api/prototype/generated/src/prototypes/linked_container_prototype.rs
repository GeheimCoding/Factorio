#[derive(serde::Deserialize)]
pub struct LinkedContainerPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    gui_mode: LinkedContainerPrototypeGuiMode,
    inventory_size: crate::types::ItemStackIndex,
    inventory_type: LinkedContainerPrototypeInventoryType,
    picture: crate::types::Sprite,
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
#[derive(serde::Deserialize)]
pub enum LinkedContainerPrototypeInventoryType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "with_bar")]
    WithBar,
    #[serde(rename = "with_filters_and_bar")]
    WithFiltersAndBar,
}

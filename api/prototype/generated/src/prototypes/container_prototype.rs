pub struct ContainerPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    default_status: crate::types::EntityStatus,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    inventory_size: crate::types::ItemStackIndex,
    inventory_type: ContainerPrototypeInventoryType,
    picture: crate::types::Sprite,
}
#[derive(serde::Deserialize)]
pub enum ContainerPrototypeInventoryType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "with_bar")]
    WithBar,
    #[serde(rename = "with_filters_and_bar")]
    WithFiltersAndBar,
}

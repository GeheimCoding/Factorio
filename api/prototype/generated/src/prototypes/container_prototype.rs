pub struct ContainerPrototype {
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    default_status: crate::types::EntityStatus,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    inventory_size: crate::types::ItemStackIndex,
    inventory_type: ContainerPrototypeInventoryType,
    picture: crate::types::Sprite,
}
pub enum ContainerPrototypeInventoryType {
    Normal,
    WithBar,
    WithFiltersAndBar,
}

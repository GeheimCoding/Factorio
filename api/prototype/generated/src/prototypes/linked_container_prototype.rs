pub struct LinkedContainerPrototype {
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    gui_mode: LinkedContainerPrototypeGuiMode,
    inventory_size: crate::types::ItemStackIndex,
    inventory_type: LinkedContainerPrototypeInventoryType,
    picture: crate::types::Sprite,
}
pub enum LinkedContainerPrototypeGuiMode {
    All,
    None,
    Admins,
}
pub enum LinkedContainerPrototypeInventoryType {
    Normal,
    WithBar,
    WithFiltersAndBar,
}

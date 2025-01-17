#[derive(serde::Deserialize)]
pub struct CargoWagonPrototype {
    base_: crate::prototypes::RollingStockPrototype,
    inventory_size: crate::types::ItemStackIndex,
}

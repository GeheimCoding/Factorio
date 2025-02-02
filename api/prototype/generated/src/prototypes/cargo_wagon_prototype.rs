#[derive(Debug, serde::Deserialize)]
pub struct CargoWagonPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RollingStockPrototype,
    inventory_size: crate::types::ItemStackIndex,
}

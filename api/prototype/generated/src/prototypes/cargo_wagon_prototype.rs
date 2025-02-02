#[derive(Debug, serde::Deserialize)]
pub struct CargoWagonPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RollingStockPrototype,
    inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_quality_affects_inventory_size")]
    quality_affects_inventory_size: bool,
}
fn default_quality_affects_inventory_size() -> bool {
    false
}

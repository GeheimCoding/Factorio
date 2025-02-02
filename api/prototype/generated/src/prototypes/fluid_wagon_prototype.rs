#[derive(Debug, serde::Deserialize)]
pub struct FluidWagonPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RollingStockPrototype,
    capacity: crate::types::FluidAmount,
    #[serde(default = "default_tank_count")]
    tank_count: u8,
}
fn default_tank_count() -> u8 {
    3
}

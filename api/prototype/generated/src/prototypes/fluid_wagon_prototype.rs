#[derive(Debug, serde::Deserialize)]
pub struct FluidWagonPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RollingStockPrototype,
    capacity: crate::types::FluidAmount,
    #[serde(default = "default_quality_affects_capacity")]
    quality_affects_capacity: bool,
    #[serde(default = "default_tank_count")]
    tank_count: u8,
}
fn default_quality_affects_capacity() -> bool {
    false
}
fn default_tank_count() -> u8 {
    3
}

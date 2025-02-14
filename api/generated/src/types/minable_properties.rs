#[derive(Debug, serde::Deserialize)]
pub struct MinableProperties {
    #[serde(default = "default_count")]
    count: u16,
    #[serde(default = "default_fluid_amount")]
    fluid_amount: crate::types::FluidAmount,
    include_in_show_counts: Option<bool>,
    mining_particle: Option<crate::types::ParticleID>,
    mining_time: f64,
    mining_trigger: Option<crate::types::Trigger>,
    required_fluid: Option<crate::types::FluidID>,
    result: Option<crate::types::ItemID>,
    results: Option<crate::vec::Vec<crate::types::ProductPrototype>>,
    #[serde(default = "default_transfer_entity_health_to_products")]
    transfer_entity_health_to_products: bool,
}
fn default_count() -> u16 {
    1
}
fn default_fluid_amount() -> crate::types::FluidAmount {
    0.0
}
fn default_transfer_entity_health_to_products() -> bool {
    true
}

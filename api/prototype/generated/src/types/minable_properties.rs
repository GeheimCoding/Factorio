#[derive(Debug, serde::Deserialize)]
pub struct MinableProperties {
    #[serde(default = "default_count")]
    count: u16,
    #[serde(default = "default_fluid_amount")]
    fluid_amount: crate::types::FluidAmount,
    mining_particle: Option<crate::types::ParticleID>,
    mining_time: f64,
    mining_trigger: Option<crate::types::Trigger>,
    required_fluid: Option<crate::types::FluidID>,
    result: Option<crate::types::ItemID>,
    results: Option<Vec<crate::types::ProductPrototype>>,
}
fn default_count() -> u16 {
    1
}
fn default_fluid_amount() -> crate::types::FluidAmount {
    0.0
}

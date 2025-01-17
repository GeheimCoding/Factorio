#[derive(serde::Deserialize)]
pub struct MinableProperties {
    count: u16,
    fluid_amount: crate::types::FluidAmount,
    mining_particle: crate::types::ParticleID,
    mining_time: f64,
    mining_trigger: crate::types::Trigger,
    required_fluid: crate::types::FluidID,
    result: crate::types::ItemID,
    results: Vec<crate::types::ProductPrototype>,
}

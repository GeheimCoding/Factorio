pub struct MinableProperties {
    count: u16,
    fluid_amount: FluidAmount,
    mining_particle: ParticleID,
    mining_time: f64,
    mining_trigger: Trigger,
    required_fluid: FluidID,
    result: ItemID,
    results: Vec<ProductPrototype>,
}

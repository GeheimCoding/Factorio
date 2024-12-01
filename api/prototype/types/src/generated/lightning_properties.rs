pub struct LightningProperties {
    exemption_rules: Vec<LightningRuleBase>,
    lightning_types: Vec<EntityID>,
    lightnings_per_chunk_per_tick: f64,
    priority_rules: Vec<LightningPriorityRule>,
    search_radius: f64,
}

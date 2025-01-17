#[derive(serde::Deserialize)]
pub struct LightningProperties {
    exemption_rules: Vec<crate::types::LightningRuleBase>,
    lightning_types: Vec<crate::types::EntityID>,
    lightnings_per_chunk_per_tick: f64,
    priority_rules: Vec<crate::types::LightningPriorityRule>,
    search_radius: f64,
}

#[derive(Debug, serde::Deserialize)]
pub struct LightningProperties {
    exemption_rules: crate::vec::Vec<crate::types::LightningRuleBase>,
    lightning_types: crate::vec::Vec<crate::types::EntityID>,
    lightnings_per_chunk_per_tick: f64,
    priority_rules: crate::vec::Vec<crate::types::LightningPriorityRule>,
    search_radius: f64,
}

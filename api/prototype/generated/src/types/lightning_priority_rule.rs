#[derive(Debug, serde::Deserialize)]
pub struct LightningPriorityRule {
    base_: crate::types::LightningRuleBase,
    priority_bonus: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct LightningPriorityRule {
    #[serde(flatten)]
    base_: crate::types::LightningRuleBase,
    priority_bonus: i32,
}

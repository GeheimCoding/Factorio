#[derive(Debug, serde::Deserialize)]
pub struct SetLogisticRequestTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    entity: Option<crate::types::EntityID>,
    #[serde(default = "default_logistic_chest_only")]
    logistic_chest_only: bool,
}
fn default_logistic_chest_only() -> bool {
    false
}

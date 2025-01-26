#[derive(Debug, serde::Deserialize)]
pub struct SetLogisticRequestTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    entity: Option<crate::types::EntityID>,
    #[serde(default = "default_logistic_chest_only")]
    logistic_chest_only: bool,
    #[serde(rename = "type")]
    type_: String,
}
fn default_logistic_chest_only() -> bool {
    false
}

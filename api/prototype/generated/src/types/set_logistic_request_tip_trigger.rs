#[derive(serde::Deserialize)]
pub struct SetLogisticRequestTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    entity: crate::types::EntityID,
    logistic_chest_only: bool,
    #[serde(rename = "type")]
    type_: String,
}

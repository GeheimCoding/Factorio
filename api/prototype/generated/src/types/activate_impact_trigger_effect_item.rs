#[derive(serde::Deserialize)]
pub struct ActivateImpactTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    deliver_category: String,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct NestedTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    action: crate::types::Trigger,
    #[serde(rename = "type")]
    type_: String,
}

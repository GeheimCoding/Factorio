#[derive(Debug, serde::Deserialize)]
pub struct NestedTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    action: crate::types::Trigger,
}

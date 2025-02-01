#[derive(Debug, serde::Deserialize)]
pub struct NestedTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    action: crate::types::Trigger,
}

#[derive(serde::Deserialize)]
pub struct NestedTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    action: crate::types::Trigger,
    type_: String,
}

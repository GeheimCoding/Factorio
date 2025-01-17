#[derive(serde::Deserialize)]
pub struct SpoilToTriggerResult {
    items_per_trigger: crate::types::ItemCountType,
    trigger: crate::types::Trigger,
}

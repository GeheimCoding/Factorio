#[derive(serde::Deserialize)]
pub struct DirectTriggerItem {
    base_: crate::types::TriggerItem,
    filter_enabled: bool,
    type_: String,
}

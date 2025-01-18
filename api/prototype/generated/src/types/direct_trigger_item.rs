#[derive(serde::Deserialize)]
pub struct DirectTriggerItem {
    base_: crate::types::TriggerItem,
    filter_enabled: bool,
    #[serde(rename = "type")]
    type_: String,
}

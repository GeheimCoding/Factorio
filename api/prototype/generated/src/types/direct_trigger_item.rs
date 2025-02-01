#[derive(Debug, serde::Deserialize)]
pub struct DirectTriggerItem {
    base_: crate::types::TriggerItem,
    #[serde(default = "default_filter_enabled")]
    filter_enabled: bool,
}
fn default_filter_enabled() -> bool {
    false
}

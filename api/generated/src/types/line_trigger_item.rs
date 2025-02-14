#[derive(Debug, serde::Deserialize)]
pub struct LineTriggerItem {
    #[serde(flatten)]
    base_: crate::types::TriggerItem,
    range: f64,
    range_effects: Option<crate::types::TriggerEffect>,
    width: f64,
}

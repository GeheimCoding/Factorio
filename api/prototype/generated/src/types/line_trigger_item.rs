#[derive(Debug, serde::Deserialize)]
pub struct LineTriggerItem {
    base_: crate::types::TriggerItem,
    range: f64,
    range_effects: Option<crate::types::TriggerEffect>,
    width: f64,
}

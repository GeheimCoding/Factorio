#[derive(serde::Deserialize)]
pub struct LineTriggerItem {
    base_: crate::types::TriggerItem,
    range: f64,
    range_effects: crate::types::TriggerEffect,
    #[serde(rename = "type")]
    type_: String,
    width: f64,
}

#[derive(serde::Deserialize)]
pub struct TriggerDeliveryItem {
    source_effects: Option<crate::types::TriggerEffect>,
    target_effects: Option<crate::types::TriggerEffect>,
}

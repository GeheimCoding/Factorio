#[derive(Debug, serde::Deserialize)]
pub struct DelayedTriggerDelivery {
    base_: crate::types::TriggerDeliveryItem,
    delayed_trigger: crate::types::ActiveTriggerID,
    #[serde(rename = "type")]
    type_: String,
}

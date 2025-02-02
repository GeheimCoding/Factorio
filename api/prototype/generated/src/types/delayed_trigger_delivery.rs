#[derive(Debug, serde::Deserialize)]
pub struct DelayedTriggerDelivery {
    #[serde(flatten)]
    base_: crate::types::TriggerDeliveryItem,
    delayed_trigger: crate::types::ActiveTriggerID,
}

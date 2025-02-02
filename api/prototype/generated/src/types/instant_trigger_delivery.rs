#[derive(Debug, serde::Deserialize)]
pub struct InstantTriggerDelivery {
    #[serde(flatten)]
    base_: crate::types::TriggerDeliveryItem,
}

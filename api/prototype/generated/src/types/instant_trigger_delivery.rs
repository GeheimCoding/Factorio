#[derive(serde::Deserialize)]
pub struct InstantTriggerDelivery {
    base_: crate::types::TriggerDeliveryItem,
    type_: String,
}

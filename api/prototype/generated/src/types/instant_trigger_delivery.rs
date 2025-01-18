#[derive(serde::Deserialize)]
pub struct InstantTriggerDelivery {
    base_: crate::types::TriggerDeliveryItem,
    #[serde(rename = "type")]
    type_: String,
}

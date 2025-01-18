#[derive(serde::Deserialize)]
pub struct StreamTriggerDelivery {
    base_: crate::types::TriggerDeliveryItem,
    source_offset: crate::types::Vector,
    stream: crate::types::EntityID,
    #[serde(rename = "type")]
    type_: String,
}

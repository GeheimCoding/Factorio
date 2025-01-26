#[derive(Debug, serde::Deserialize)]
pub struct StreamTriggerDelivery {
    base_: crate::types::TriggerDeliveryItem,
    source_offset: Option<crate::types::Vector>,
    stream: crate::types::EntityID,
    #[serde(rename = "type")]
    type_: String,
}

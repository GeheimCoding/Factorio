#[derive(Debug, serde::Deserialize)]
pub struct StreamTriggerDelivery {
    #[serde(flatten)]
    base_: crate::types::TriggerDeliveryItem,
    source_offset: Option<crate::types::Vector>,
    stream: crate::types::EntityID,
}

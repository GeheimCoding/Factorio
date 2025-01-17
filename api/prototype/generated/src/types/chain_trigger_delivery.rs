#[derive(serde::Deserialize)]
pub struct ChainTriggerDelivery {
    base_: crate::types::TriggerDeliveryItem,
    chain: crate::types::ActiveTriggerID,
    type_: String,
}

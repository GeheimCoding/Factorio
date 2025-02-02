#[derive(Debug, serde::Deserialize)]
pub struct ChainTriggerDelivery {
    #[serde(flatten)]
    base_: crate::types::TriggerDeliveryItem,
    chain: crate::types::ActiveTriggerID,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityTransferTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    // default: any transfer
    transfer: Option<EntityTransferTipTriggerTransfer>,
}
#[derive(Debug, serde::Deserialize)]
pub enum EntityTransferTipTriggerTransfer {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

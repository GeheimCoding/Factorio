#[derive(serde::Deserialize)]
pub struct EntityTransferTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    // default: any transfer
    transfer: EntityTransferTipTriggerTransfer,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub enum EntityTransferTipTriggerTransfer {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

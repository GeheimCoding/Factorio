pub struct EntityTransferTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    transfer: EntityTransferTipTriggerTransfer,
    type_: String,
}
#[derive(serde::Deserialize)]
pub enum EntityTransferTipTriggerTransfer {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

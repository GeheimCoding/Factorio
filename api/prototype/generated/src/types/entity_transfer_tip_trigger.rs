pub struct EntityTransferTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    transfer: EntityTransferTipTriggerTransfer,
    type_: String,
}
pub enum EntityTransferTipTriggerTransfer {
    In,
    Out,
}

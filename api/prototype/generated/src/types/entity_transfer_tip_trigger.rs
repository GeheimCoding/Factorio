pub struct EntityTransferTipTrigger {
    transfer: EntityTransferTipTriggerTransfer,
    type_: String,
}
pub enum EntityTransferTipTriggerTransfer {
    In,
    Out,
}

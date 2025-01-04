pub struct EntityTransferTipTrigger {
    transfer: EntityTransferTipTriggerTransfer,
    type_: EntityTransfer,
}
pub enum EntityTransferTipTriggerTransfer {
    In,
    Out,
}

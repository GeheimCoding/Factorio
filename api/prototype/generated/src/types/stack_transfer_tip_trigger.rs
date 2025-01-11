pub struct StackTransferTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    transfer: StackTransferTipTriggerTransfer,
    type_: String,
}
pub enum StackTransferTipTriggerTransfer {
    Stack,
    Inventory,
    WholeInventory,
}

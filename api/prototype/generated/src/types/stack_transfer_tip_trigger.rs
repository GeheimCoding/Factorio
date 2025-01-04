pub struct StackTransferTipTrigger {
    transfer: StackTransferTipTriggerTransfer,
    type_: StackTransfer,
}
pub enum StackTransferTipTriggerTransfer {
    Stack,
    Inventory,
    WholeInventory,
}

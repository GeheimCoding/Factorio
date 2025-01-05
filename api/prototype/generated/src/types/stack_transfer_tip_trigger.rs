pub struct StackTransferTipTrigger {
    transfer: StackTransferTipTriggerTransfer,
    type_: String,
}
pub enum StackTransferTipTriggerTransfer {
    Stack,
    Inventory,
    WholeInventory,
}

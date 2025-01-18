#[derive(serde::Deserialize)]
pub struct StackTransferTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    // default: any transfer
    transfer: StackTransferTipTriggerTransfer,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub enum StackTransferTipTriggerTransfer {
    #[serde(rename = "stack")]
    Stack,
    #[serde(rename = "inventory")]
    Inventory,
    #[serde(rename = "whole_inventory")]
    WholeInventory,
}

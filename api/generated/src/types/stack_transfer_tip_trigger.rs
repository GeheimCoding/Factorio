#[derive(Debug, serde::Deserialize)]
pub struct StackTransferTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    // default: any transfer
    transfer: Option<StackTransferTipTriggerTransfer>,
}
#[derive(Debug, serde::Deserialize)]
pub enum StackTransferTipTriggerTransfer {
    #[serde(rename = "stack")]
    Stack,
    #[serde(rename = "inventory")]
    Inventory,
    #[serde(rename = "whole-inventory")]
    WholeInventory,
}

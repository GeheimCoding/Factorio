#[derive(Debug, serde::Deserialize)]
pub struct ModuleTransferTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    module: crate::types::ItemID,
}

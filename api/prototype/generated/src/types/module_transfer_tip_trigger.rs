#[derive(Debug, serde::Deserialize)]
pub struct ModuleTransferTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    module: crate::types::ItemID,
}

#[derive(serde::Deserialize)]
pub struct ModuleTransferTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    module: crate::types::ItemID,
    #[serde(rename = "type")]
    type_: String,
}

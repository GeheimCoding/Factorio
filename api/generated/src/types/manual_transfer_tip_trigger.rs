#[derive(Debug, serde::Deserialize)]
pub struct ManualTransferTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}

#[derive(Debug, serde::Deserialize)]
pub struct ManualTransferTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
}

#[derive(serde::Deserialize)]
pub struct ManualTransferTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}

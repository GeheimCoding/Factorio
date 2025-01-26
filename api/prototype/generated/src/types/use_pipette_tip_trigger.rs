#[derive(Debug, serde::Deserialize)]
pub struct UsePipetteTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}

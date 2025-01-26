#[derive(Debug, serde::Deserialize)]
pub struct ClearCursorTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}

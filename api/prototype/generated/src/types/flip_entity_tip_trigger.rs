#[derive(Debug, serde::Deserialize)]
pub struct FlipEntityTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}

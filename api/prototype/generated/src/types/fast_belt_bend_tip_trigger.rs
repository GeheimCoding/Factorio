#[derive(Debug, serde::Deserialize)]
pub struct FastBeltBendTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}

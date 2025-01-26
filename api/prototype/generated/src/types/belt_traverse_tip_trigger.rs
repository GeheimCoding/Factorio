#[derive(Debug, serde::Deserialize)]
pub struct BeltTraverseTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}

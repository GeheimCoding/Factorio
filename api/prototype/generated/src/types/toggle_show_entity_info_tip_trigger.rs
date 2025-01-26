#[derive(Debug, serde::Deserialize)]
pub struct ToggleShowEntityInfoTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}

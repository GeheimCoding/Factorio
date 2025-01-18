#[derive(serde::Deserialize)]
pub struct OrTipTrigger {
    triggers: Vec<crate::types::TipTrigger>,
    #[serde(rename = "type")]
    type_: String,
}

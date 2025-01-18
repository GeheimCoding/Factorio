#[derive(serde::Deserialize)]
pub struct AndTipTrigger {
    triggers: Vec<crate::types::TipTrigger>,
    #[serde(rename = "type")]
    type_: String,
}

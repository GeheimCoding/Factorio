#[derive(serde::Deserialize)]
pub struct SequenceTipTrigger {
    triggers: Vec<crate::types::TipTrigger>,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(serde::Deserialize)]
pub struct SequenceTipTrigger {
    triggers: Vec<crate::types::TipTrigger>,
    type_: String,
}

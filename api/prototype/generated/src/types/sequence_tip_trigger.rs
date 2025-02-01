#[derive(Debug, serde::Deserialize)]
pub struct SequenceTipTrigger {
    triggers: Vec<crate::types::TipTrigger>,
}

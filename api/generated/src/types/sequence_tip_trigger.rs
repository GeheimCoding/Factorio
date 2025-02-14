#[derive(Debug, serde::Deserialize)]
pub struct SequenceTipTrigger {
    triggers: crate::vec::Vec<crate::types::TipTrigger>,
}

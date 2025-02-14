#[derive(Debug, serde::Deserialize)]
pub struct AndTipTrigger {
    triggers: crate::vec::Vec<crate::types::TipTrigger>,
}

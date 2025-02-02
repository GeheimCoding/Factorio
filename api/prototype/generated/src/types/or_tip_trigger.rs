#[derive(Debug, serde::Deserialize)]
pub struct OrTipTrigger {
    triggers: crate::vec::Vec<crate::types::TipTrigger>,
}

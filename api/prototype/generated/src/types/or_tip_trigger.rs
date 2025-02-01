#[derive(Debug, serde::Deserialize)]
pub struct OrTipTrigger {
    triggers: Vec<crate::types::TipTrigger>,
}

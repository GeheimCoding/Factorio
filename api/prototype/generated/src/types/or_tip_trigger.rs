#[derive(serde::Deserialize)]
pub struct OrTipTrigger {
    triggers: Vec<crate::types::TipTrigger>,
    type_: String,
}

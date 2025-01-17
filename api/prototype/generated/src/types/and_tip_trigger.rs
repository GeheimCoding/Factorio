#[derive(serde::Deserialize)]
pub struct AndTipTrigger {
    triggers: Vec<crate::types::TipTrigger>,
    type_: String,
}

#[derive(serde::Deserialize)]
pub struct ActivatePasteTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}

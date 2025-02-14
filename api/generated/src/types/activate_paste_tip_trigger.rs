#[derive(Debug, serde::Deserialize)]
pub struct ActivatePasteTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}

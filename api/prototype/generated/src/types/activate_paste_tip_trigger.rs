#[derive(serde::Deserialize)]
pub struct ActivatePasteTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}

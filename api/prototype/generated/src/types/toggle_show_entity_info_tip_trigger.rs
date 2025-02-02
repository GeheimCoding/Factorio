#[derive(Debug, serde::Deserialize)]
pub struct ToggleShowEntityInfoTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}

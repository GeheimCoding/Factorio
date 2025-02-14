#[derive(Debug, serde::Deserialize)]
pub struct RotateEntityTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}

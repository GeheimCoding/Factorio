#[derive(Debug, serde::Deserialize)]
pub struct FlipEntityTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}

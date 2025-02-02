#[derive(Debug, serde::Deserialize)]
pub struct BeltTraverseTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}

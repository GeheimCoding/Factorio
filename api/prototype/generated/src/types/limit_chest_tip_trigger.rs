#[derive(Debug, serde::Deserialize)]
pub struct LimitChestTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}

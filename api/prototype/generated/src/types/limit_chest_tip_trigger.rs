#[derive(serde::Deserialize)]
pub struct LimitChestTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}

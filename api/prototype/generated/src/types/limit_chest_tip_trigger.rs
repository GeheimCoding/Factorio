#[derive(serde::Deserialize)]
pub struct LimitChestTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}

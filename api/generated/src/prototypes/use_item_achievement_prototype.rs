#[derive(Debug, serde::Deserialize)]
pub struct UseItemAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_amount")]
    amount: u32,
    limit_quality: crate::types::QualityID,
    #[serde(default = "default_limited_to_one_game")]
    limited_to_one_game: bool,
    to_use: crate::types::ItemID,
}
fn default_amount() -> u32 {
    1
}
fn default_limited_to_one_game() -> bool {
    false
}

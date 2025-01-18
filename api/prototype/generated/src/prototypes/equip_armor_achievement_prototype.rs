#[derive(serde::Deserialize)]
pub struct EquipArmorAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    alternative_armor: crate::types::ItemID,
    #[serde(default = "default_amount")]
    amount: u32,
    armor: crate::types::ItemID,
    limit_quality: crate::types::QualityID,
    #[serde(default = "default_limited_to_one_game")]
    limited_to_one_game: bool,
}
fn default_amount() -> u32 {
    1
}
fn default_limited_to_one_game() -> bool {
    false
}

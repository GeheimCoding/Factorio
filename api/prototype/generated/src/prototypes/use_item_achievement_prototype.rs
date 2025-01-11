pub struct UseItemAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    amount: u32,
    limit_quality: crate::types::QualityID,
    limited_to_one_game: bool,
    to_use: crate::types::ItemID,
}

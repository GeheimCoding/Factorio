pub struct EquipArmorAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    alternative_armor: crate::types::ItemID,
    amount: u32,
    armor: crate::types::ItemID,
    limit_quality: crate::types::QualityID,
    limited_to_one_game: bool,
}

pub struct PlaceEquipmentAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    amount: u32,
    armor: crate::types::ItemID,
    limit_equip_quality: crate::types::QualityID,
    limit_quality: crate::types::QualityID,
    limited_to_one_game: bool,
}

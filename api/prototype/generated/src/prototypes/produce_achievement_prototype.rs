pub struct ProduceAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    amount: crate::types::MaterialAmountType,
    fluid_product: crate::types::FluidID,
    item_product: crate::types::ItemID,
    limited_to_one_game: bool,
    quality: crate::types::QualityID,
}

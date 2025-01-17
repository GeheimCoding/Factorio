#[derive(serde::Deserialize)]
pub struct ProducePerHourAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    amount: crate::types::MaterialAmountType,
    fluid_product: crate::types::FluidID,
    item_product: crate::types::ItemID,
}

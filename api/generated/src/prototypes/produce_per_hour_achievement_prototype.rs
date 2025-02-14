#[derive(Debug, serde::Deserialize)]
pub struct ProducePerHourAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    amount: crate::types::MaterialAmountType,
    fluid_product: Option<crate::types::FluidID>,
    item_product: Option<crate::types::ItemIDFilter>,
}

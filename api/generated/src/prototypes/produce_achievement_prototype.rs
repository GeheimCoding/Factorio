#[derive(Debug, serde::Deserialize)]
pub struct ProduceAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    amount: crate::types::MaterialAmountType,
    fluid_product: Option<crate::types::FluidID>,
    item_product: Option<crate::types::ItemIDFilter>,
    limited_to_one_game: bool,
}

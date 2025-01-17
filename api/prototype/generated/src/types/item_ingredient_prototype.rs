#[derive(serde::Deserialize)]
pub struct ItemIngredientPrototype {
    amount: u16,
    ignored_by_stats: u16,
    name: crate::types::ItemID,
    type_: String,
}

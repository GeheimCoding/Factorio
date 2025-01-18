#[derive(serde::Deserialize)]
pub struct ItemIngredientPrototype {
    amount: u16,
    #[serde(default = "default_ignored_by_stats")]
    ignored_by_stats: u16,
    name: crate::types::ItemID,
    #[serde(rename = "type")]
    type_: String,
}
fn default_ignored_by_stats() -> u16 {
    0
}

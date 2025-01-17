#[derive(serde::Deserialize)]
pub enum IngredientPrototype {
    #[serde(untagged)]
    ItemIngredientPrototype(Box<crate::types::ItemIngredientPrototype>),
    #[serde(untagged)]
    FluidIngredientPrototype(Box<crate::types::FluidIngredientPrototype>),
}

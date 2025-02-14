#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum IngredientPrototype {
    #[serde(rename = "item")]
    ItemIngredientPrototype(Box<crate::types::ItemIngredientPrototype>),
    #[serde(rename = "fluid")]
    FluidIngredientPrototype(Box<crate::types::FluidIngredientPrototype>),
}

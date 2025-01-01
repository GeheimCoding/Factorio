pub enum IngredientPrototype {
    ItemIngredientPrototype(Box<ItemIngredientPrototype>),
    FluidIngredientPrototype(Box<FluidIngredientPrototype>),
}

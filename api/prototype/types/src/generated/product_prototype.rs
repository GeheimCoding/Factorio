pub enum ProductPrototype {
    ItemProductPrototype(Box<ItemProductPrototype>),
    FluidProductPrototype(Box<FluidProductPrototype>),
    ResearchProgressProductPrototype(Box<ResearchProgressProductPrototype>),
}

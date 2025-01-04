pub enum ProductPrototype {
    ItemProductPrototype(Box<crate::types::ItemProductPrototype>),
    FluidProductPrototype(Box<crate::types::FluidProductPrototype>),
    ResearchProgressProductPrototype(Box<crate::types::ResearchProgressProductPrototype>),
}

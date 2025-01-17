#[derive(serde::Deserialize)]
pub enum ProductPrototype {
    #[serde(untagged)]
    ItemProductPrototype(Box<crate::types::ItemProductPrototype>),
    #[serde(untagged)]
    FluidProductPrototype(Box<crate::types::FluidProductPrototype>),
    #[serde(untagged)]
    ResearchProgressProductPrototype(Box<crate::types::ResearchProgressProductPrototype>),
}

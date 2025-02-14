#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ProductPrototype {
    #[serde(rename = "item")]
    ItemProductPrototype(Box<crate::types::ItemProductPrototype>),
    #[serde(rename = "fluid")]
    FluidProductPrototype(Box<crate::types::FluidProductPrototype>),
    #[serde(rename = "research-progress")]
    ResearchProgressProductPrototype(Box<crate::types::ResearchProgressProductPrototype>),
}

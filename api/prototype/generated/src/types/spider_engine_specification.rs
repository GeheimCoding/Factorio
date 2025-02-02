#[derive(Debug, serde::Deserialize)]
pub struct SpiderEngineSpecification {
    legs: SpiderEngineSpecificationLegs,
    #[serde(default = "default_walking_group_overlap")]
    walking_group_overlap: f32,
}
#[derive(Debug, serde::Deserialize)]
pub enum SpiderEngineSpecificationLegs {
    #[serde(untagged)]
    SpiderLegSpecification(Box<crate::types::SpiderLegSpecification>),
    #[serde(untagged)]
    VecSpiderLegSpecification(crate::vec::Vec<crate::types::SpiderLegSpecification>),
}
fn default_walking_group_overlap() -> f32 {
    0.0
}

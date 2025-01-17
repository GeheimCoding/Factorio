pub struct SpiderEngineSpecification {
    legs: SpiderEngineSpecificationLegs,
    walking_group_overlap: f32,
}
#[derive(serde::Deserialize)]
pub enum SpiderEngineSpecificationLegs {
    #[serde(untagged)]
    SpiderLegSpecification(Box<crate::types::SpiderLegSpecification>),
    #[serde(untagged)]
    VecSpiderLegSpecification(Vec<crate::types::SpiderLegSpecification>),
}

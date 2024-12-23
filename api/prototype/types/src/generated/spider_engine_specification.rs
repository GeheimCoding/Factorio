pub struct SpiderEngineSpecification {
    legs: SpiderEngineSpecificationLegs,
    walking_group_overlap: f32,
}
pub enum SpiderEngineSpecificationLegs {
    SpiderLegSpecification(SpiderLegSpecification),
    VecSpiderLegSpecification(Vec<SpiderLegSpecification>),
}

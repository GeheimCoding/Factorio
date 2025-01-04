pub struct PlumesSpecification {
    max_probability: f32,
    max_y_offset: f32,
    min_probability: f32,
    min_y_offset: f32,
    stateless_visualisations: PlumesSpecificationStatelessVisualisations,
}
pub enum PlumesSpecificationStatelessVisualisations {
    PlumeEffect(Box<PlumeEffect>),
    VecPlumeEffect(Vec<PlumeEffect>),
}

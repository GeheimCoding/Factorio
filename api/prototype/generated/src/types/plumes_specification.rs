#[derive(serde::Deserialize)]
pub struct PlumesSpecification {
    max_probability: f32,
    max_y_offset: f32,
    min_probability: f32,
    min_y_offset: f32,
    stateless_visualisations: PlumesSpecificationStatelessVisualisations,
}
#[derive(serde::Deserialize)]
pub enum PlumesSpecificationStatelessVisualisations {
    #[serde(untagged)]
    PlumeEffect(Box<crate::types::PlumeEffect>),
    #[serde(untagged)]
    VecPlumeEffect(Vec<crate::types::PlumeEffect>),
}

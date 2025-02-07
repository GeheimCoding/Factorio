#[derive(Debug, serde::Deserialize)]
pub struct PlumesSpecification {
    #[serde(default = "default_max_probability")]
    max_probability: f32,
    #[serde(default = "default_max_y_offset")]
    max_y_offset: f32,
    #[serde(default = "default_min_probability")]
    min_probability: f32,
    #[serde(default = "default_min_y_offset")]
    min_y_offset: f32,
    render_box: Option<crate::types::BoundingBox>,
    stateless_visualisations: Option<PlumesSpecificationStatelessVisualisations>,
}
fn default_max_probability() -> f32 {
    1.0
}
fn default_max_y_offset() -> f32 {
    0.0
}
fn default_min_probability() -> f32 {
    0.0
}
fn default_min_y_offset() -> f32 {
    0.0
}
#[derive(Debug, serde::Deserialize)]
pub enum PlumesSpecificationStatelessVisualisations {
    #[serde(untagged)]
    PlumeEffect(Box<crate::types::PlumeEffect>),
    #[serde(untagged)]
    VecPlumeEffect(crate::vec::Vec<crate::types::PlumeEffect>),
}

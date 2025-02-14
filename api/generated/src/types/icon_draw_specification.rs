#[derive(Debug, serde::Deserialize)]
pub struct IconDrawSpecification {
    #[serde(rename = "renderLayer")]
    #[serde(default = "default_render_layer")]
    render_layer: IconDrawSpecificationRenderLayer,
    #[serde(default = "default_scale")]
    scale: f32,
    #[serde(default = "default_scale_for_many")]
    scale_for_many: f32,
    // default: `{0, 0}`
    shift: Option<crate::types::Vector>,
}
#[derive(Debug, serde::Deserialize)]
pub enum IconDrawSpecificationRenderLayer {
    #[serde(rename = "entity-info-icon-below")]
    EntityInfoIconBelow,
    #[serde(rename = "entity-info-icon-above")]
    EntityInfoIconAbove,
    #[serde(rename = "air-entity-info-icon")]
    AirEntityInfoIcon,
}
fn default_render_layer() -> IconDrawSpecificationRenderLayer {
    IconDrawSpecificationRenderLayer::EntityInfoIconBelow
}
fn default_scale() -> f32 {
    1.0
}
fn default_scale_for_many() -> f32 {
    1.0
}

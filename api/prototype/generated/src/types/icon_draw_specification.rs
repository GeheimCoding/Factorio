pub struct IconDrawSpecification {
    renderLayer: IconDrawSpecificationRenderLayer,
    scale: f32,
    scale_for_many: f32,
    shift: crate::types::Vector,
}
#[derive(serde::Deserialize)]
pub enum IconDrawSpecificationRenderLayer {
    #[serde(rename = "entity_info_icon_below")]
    EntityInfoIconBelow,
    #[serde(rename = "entity_info_icon_above")]
    EntityInfoIconAbove,
    #[serde(rename = "air_entity_info_icon")]
    AirEntityInfoIcon,
}

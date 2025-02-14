#[derive(Debug, serde::Deserialize)]
pub struct ProcessionGraphic {
    animation: Option<crate::types::Animation>,
    catalogue_id: Option<u32>,
    sprite: Option<crate::types::Sprite>,
    #[serde(rename = "type")]
    type_: crate::types::ProcessionGraphicType,
}

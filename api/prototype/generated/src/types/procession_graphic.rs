#[derive(serde::Deserialize)]
pub struct ProcessionGraphic {
    animation: crate::types::Animation,
    catalogue_id: u32,
    sprite: crate::types::Sprite,
    #[serde(rename = "type")]
    type_: crate::types::ProcessionGraphicType,
}

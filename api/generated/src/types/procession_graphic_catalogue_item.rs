#[derive(Debug, serde::Deserialize)]
pub struct ProcessionGraphicCatalogueItem {
    animation: Option<crate::types::Animation>,
    index: u32,
    picture: Option<crate::types::Sprite>,
}

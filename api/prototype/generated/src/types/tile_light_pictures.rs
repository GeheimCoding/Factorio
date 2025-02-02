#[derive(Debug, serde::Deserialize)]
pub struct TileLightPictures {
    #[serde(flatten)]
    base_: crate::types::TileSpriteLayout,
    size: u32,
}

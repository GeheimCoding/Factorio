#[derive(serde::Deserialize)]
pub enum SpriteVariations {
    #[serde(untagged)]
    SpriteVariations { sheet: crate::types::SpriteSheet },
    #[serde(untagged)]
    SpriteSheet(Box<crate::types::SpriteSheet>),
    #[serde(untagged)]
    VecSprite(Vec<crate::types::Sprite>),
}

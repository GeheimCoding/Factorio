#[derive(Debug, serde::Deserialize)]
pub enum SpriteVariations {
    #[serde(untagged)]
    SpriteVariations {
        sheet: Box<crate::types::SpriteSheet>,
    },
    #[serde(untagged)]
    SpriteSheet(Box<crate::types::SpriteSheet>),
    #[serde(untagged)]
    VecSprite(crate::vec::Vec<crate::types::Sprite>),
}

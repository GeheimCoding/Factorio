pub enum SpriteVariations {
    SpriteVariations { sheet: crate::types::SpriteSheet },
    SpriteSheet(Box<crate::types::SpriteSheet>),
    VecSprite(Vec<crate::types::Sprite>),
}

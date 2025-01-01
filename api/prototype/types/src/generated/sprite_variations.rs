pub enum SpriteVariations {
    SpriteVariations { sheet: SpriteSheet },
    SpriteSheet(Box<SpriteSheet>),
    VecSprite(Vec<Sprite>),
}

pub enum SpriteVariations {
    SpriteVariations { sheet: SpriteSheet },
    SpriteSheet(SpriteSheet),
    VecSprite(Vec<Sprite>),
}

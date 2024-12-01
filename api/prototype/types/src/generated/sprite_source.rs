pub struct SpriteSource {
    filename: FileName,
    height: SpriteSizeType,
    load_in_minimal_mode: bool,
    position: (SpriteSizeType, SpriteSizeType),
    premul_alpha: bool,
    size: SpriteSourceSize,
    width: SpriteSizeType,
    x: SpriteSizeType,
    y: SpriteSizeType,
}
pub enum SpriteSourceSize {
    SpriteSizeType(SpriteSizeType),
    SpriteSizeTypeSpriteSizeType((SpriteSizeType, SpriteSizeType)),
}

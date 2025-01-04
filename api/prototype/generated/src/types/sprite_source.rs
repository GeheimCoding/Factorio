pub struct SpriteSource {
    filename: crate::types::FileName,
    height: crate::types::SpriteSizeType,
    load_in_minimal_mode: bool,
    position: (crate::types::SpriteSizeType, crate::types::SpriteSizeType),
    premul_alpha: bool,
    size: SpriteSourceSize,
    width: crate::types::SpriteSizeType,
    x: crate::types::SpriteSizeType,
    y: crate::types::SpriteSizeType,
}
pub enum SpriteSourceSize {
    SpriteSizeType(crate::types::SpriteSizeType),
    SpriteSizeTypeSpriteSizeType((crate::types::SpriteSizeType, crate::types::SpriteSizeType)),
}

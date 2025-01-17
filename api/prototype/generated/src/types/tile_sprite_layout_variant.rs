#[derive(serde::Deserialize)]
pub struct TileSpriteLayoutVariant {
    count: u8,
    line_length: u8,
    scale: f32,
    spritesheet: crate::types::FileName,
    tile_height: u8,
    x: crate::types::SpriteSizeType,
    y: crate::types::SpriteSizeType,
}

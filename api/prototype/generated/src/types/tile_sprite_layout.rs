#[derive(serde::Deserialize)]
pub struct TileSpriteLayout {
    count: u8,
    line_length: u8,
    picture: crate::types::FileName,
    scale: f32,
    x: crate::types::SpriteSizeType,
    y: crate::types::SpriteSizeType,
}

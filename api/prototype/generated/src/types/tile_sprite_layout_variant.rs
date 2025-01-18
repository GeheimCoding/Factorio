#[derive(serde::Deserialize)]
pub struct TileSpriteLayoutVariant {
    count: u8,
    #[serde(default = "default_line_length")]
    line_length: u8,
    scale: f32,
    spritesheet: crate::types::FileName,
    #[serde(default = "default_tile_height")]
    tile_height: u8,
    x: crate::types::SpriteSizeType,
    y: crate::types::SpriteSizeType,
}
fn default_line_length() -> u8 {
    0
}
fn default_tile_height() -> u8 {
    1
}

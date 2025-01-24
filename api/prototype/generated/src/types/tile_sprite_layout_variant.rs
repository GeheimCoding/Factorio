#[derive(serde::Deserialize)]
pub struct TileSpriteLayoutVariant {
    count: Option<u8>,
    #[serde(default = "default_line_length")]
    line_length: u8,
    scale: Option<f32>,
    spritesheet: Option<crate::types::FileName>,
    #[serde(default = "default_tile_height")]
    tile_height: u8,
    x: Option<crate::types::SpriteSizeType>,
    y: Option<crate::types::SpriteSizeType>,
}
fn default_line_length() -> u8 {
    0
}
fn default_tile_height() -> u8 {
    1
}

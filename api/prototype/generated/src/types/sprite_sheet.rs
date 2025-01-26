#[derive(Debug, serde::Deserialize)]
pub struct SpriteSheet {
    base_: crate::types::SpriteParameters,
    dice: Option<crate::types::SpriteSizeType>,
    dice_x: Option<crate::types::SpriteSizeType>,
    dice_y: Option<crate::types::SpriteSizeType>,
    filenames: Option<Vec<crate::types::FileName>>,
    layers: Option<Vec<crate::types::SpriteSheet>>,
    // default: Value of `variation_count`
    line_length: Option<u32>,
    lines_per_file: Option<u32>,
    #[serde(default = "default_repeat_count")]
    repeat_count: u32,
    #[serde(default = "default_variation_count")]
    variation_count: u32,
}
fn default_repeat_count() -> u32 {
    1
}
fn default_variation_count() -> u32 {
    1
}

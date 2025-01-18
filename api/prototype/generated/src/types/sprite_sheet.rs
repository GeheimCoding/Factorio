#[derive(serde::Deserialize)]
pub struct SpriteSheet {
    base_: crate::types::SpriteParameters,
    dice: crate::types::SpriteSizeType,
    dice_x: crate::types::SpriteSizeType,
    dice_y: crate::types::SpriteSizeType,
    filenames: Vec<crate::types::FileName>,
    layers: Vec<crate::types::SpriteSheet>,
    // default: Value of `variation_count`
    line_length: u32,
    lines_per_file: u32,
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

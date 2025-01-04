pub struct SpriteSheet {
    dice: crate::types::SpriteSizeType,
    dice_x: crate::types::SpriteSizeType,
    dice_y: crate::types::SpriteSizeType,
    filenames: Vec<crate::types::FileName>,
    layers: Vec<crate::types::SpriteSheet>,
    line_length: u32,
    lines_per_file: u32,
    repeat_count: u32,
    variation_count: u32,
}

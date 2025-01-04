pub struct SpriteSheet {
    dice: SpriteSizeType,
    dice_x: SpriteSizeType,
    dice_y: SpriteSizeType,
    filenames: Vec<FileName>,
    layers: Vec<SpriteSheet>,
    line_length: u32,
    lines_per_file: u32,
    repeat_count: u32,
    variation_count: u32,
}

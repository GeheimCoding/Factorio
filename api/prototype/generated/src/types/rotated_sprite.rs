#[derive(serde::Deserialize)]
pub struct RotatedSprite {
    base_: crate::types::SpriteParameters,
    #[serde(default = "default_allow_low_quality_rotation")]
    allow_low_quality_rotation: bool,
    #[serde(default = "default_apply_projection")]
    apply_projection: bool,
    #[serde(default = "default_axially_symmetrical")]
    axially_symmetrical: bool,
    #[serde(default = "default_back_equals_front")]
    back_equals_front: bool,
    #[serde(default = "default_counterclockwise")]
    counterclockwise: bool,
    dice: crate::types::SpriteSizeType,
    dice_x: crate::types::SpriteSizeType,
    dice_y: crate::types::SpriteSizeType,
    direction_count: u16,
    filename: crate::types::FileName,
    filenames: Vec<crate::types::FileName>,
    frames: Vec<crate::types::RotatedSpriteFrame>,
    #[serde(default = "default_generate_sdf")]
    generate_sdf: bool,
    layers: Vec<crate::types::RotatedSprite>,
    #[serde(default = "default_line_length")]
    line_length: u32,
    #[serde(default = "default_lines_per_file")]
    lines_per_file: u64,
}
fn default_allow_low_quality_rotation() -> bool {
    false
}
fn default_apply_projection() -> bool {
    true
}
fn default_axially_symmetrical() -> bool {
    false
}
fn default_back_equals_front() -> bool {
    false
}
fn default_counterclockwise() -> bool {
    false
}
fn default_generate_sdf() -> bool {
    false
}
fn default_line_length() -> u32 {
    0
}
fn default_lines_per_file() -> u64 {
    0
}

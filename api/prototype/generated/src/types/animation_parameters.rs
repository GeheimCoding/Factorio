#[derive(serde::Deserialize)]
pub struct AnimationParameters {
    base_: crate::types::SpriteParameters,
    #[serde(default = "default_animation_speed")]
    animation_speed: f32,
    dice: u8,
    dice_x: u8,
    dice_y: u8,
    #[serde(default = "default_frame_count")]
    frame_count: u32,
    frame_sequence: crate::types::AnimationFrameSequence,
    #[serde(default = "default_generate_sdf")]
    generate_sdf: bool,
    height: crate::types::SpriteSizeType,
    #[serde(default = "default_line_length")]
    line_length: u32,
    // default: MAX_FLOAT
    max_advance: f32,
    #[serde(default = "default_mipmap_count")]
    mipmap_count: u8,
    #[serde(default = "default_repeat_count")]
    repeat_count: u8,
    #[serde(default = "default_run_mode")]
    run_mode: crate::types::AnimationRunMode,
    size: AnimationParametersSize,
    width: crate::types::SpriteSizeType,
}
fn default_animation_speed() -> f32 {
    1.0
}
fn default_frame_count() -> u32 {
    1
}
fn default_generate_sdf() -> bool {
    false
}
fn default_line_length() -> u32 {
    0
}
fn default_mipmap_count() -> u8 {
    0
}
fn default_repeat_count() -> u8 {
    1
}
fn default_run_mode() -> crate::types::AnimationRunMode {
    crate::types::AnimationRunMode::Forward
}
#[derive(serde::Deserialize)]
pub enum AnimationParametersSize {
    #[serde(untagged)]
    SpriteSizeType(crate::types::SpriteSizeType),
    #[serde(untagged)]
    SpriteSizeTypeSpriteSizeType((crate::types::SpriteSizeType, crate::types::SpriteSizeType)),
}

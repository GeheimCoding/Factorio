pub struct AnimationParameters {
    animation_speed: f32,
    dice: u8,
    dice_x: u8,
    dice_y: u8,
    frame_count: u32,
    frame_sequence: AnimationFrameSequence,
    generate_sdf: bool,
    height: SpriteSizeType,
    line_length: u32,
    max_advance: f32,
    mipmap_count: u8,
    repeat_count: u8,
    run_mode: AnimationRunMode,
    size: AnimationParametersSize,
    width: SpriteSizeType,
}
pub enum AnimationParametersSize {
    SpriteSizeType(SpriteSizeType),
    SpriteSizeTypeSpriteSizeType((SpriteSizeType, SpriteSizeType)),
}

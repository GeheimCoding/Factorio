pub struct AnimationParameters {
    base_: crate::types::SpriteParameters,
    animation_speed: f32,
    dice: u8,
    dice_x: u8,
    dice_y: u8,
    frame_count: u32,
    frame_sequence: crate::types::AnimationFrameSequence,
    generate_sdf: bool,
    height: crate::types::SpriteSizeType,
    line_length: u32,
    max_advance: f32,
    mipmap_count: u8,
    repeat_count: u8,
    run_mode: crate::types::AnimationRunMode,
    size: AnimationParametersSize,
    width: crate::types::SpriteSizeType,
}
pub enum AnimationParametersSize {
    SpriteSizeType(crate::types::SpriteSizeType),
    SpriteSizeTypeSpriteSizeType((crate::types::SpriteSizeType, crate::types::SpriteSizeType)),
}

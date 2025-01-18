#[derive(serde::Deserialize)]
pub struct SpritePrototype {
    apply_runtime_tint: bool,
    apply_special_effect: bool,
    blend_mode: crate::types::BlendMode,
    dice: crate::types::SpriteSizeType,
    dice_x: crate::types::SpriteSizeType,
    dice_y: crate::types::SpriteSizeType,
    draw_as_glow: bool,
    draw_as_light: bool,
    draw_as_shadow: bool,
    filename: crate::types::FileName,
    flags: crate::types::SpriteFlags,
    generate_sdf: bool,
    height: crate::types::SpriteSizeType,
    invert_colors: bool,
    layers: Vec<crate::types::Sprite>,
    load_in_minimal_mode: bool,
    mipmap_count: u8,
    name: String,
    position: (crate::types::SpriteSizeType, crate::types::SpriteSizeType),
    premul_alpha: bool,
    priority: crate::types::SpritePriority,
    rotate_shift: bool,
    scale: f64,
    shift: crate::types::Vector,
    size: SpritePrototypeSize,
    surface: crate::types::SpriteUsageSurfaceHint,
    tint: crate::types::Color,
    tint_as_overlay: bool,
    #[serde(rename = "type")]
    type_: String,
    usage: crate::types::SpriteUsageHint,
    width: crate::types::SpriteSizeType,
    x: crate::types::SpriteSizeType,
    y: crate::types::SpriteSizeType,
}
#[derive(serde::Deserialize)]
pub enum SpritePrototypeSize {
    #[serde(untagged)]
    SpriteSizeType(crate::types::SpriteSizeType),
    #[serde(untagged)]
    SpriteSizeTypeSpriteSizeType((crate::types::SpriteSizeType, crate::types::SpriteSizeType)),
}

#[derive(serde::Deserialize)]
pub struct Sprite {
    base_: crate::types::SpriteParameters,
    dice: crate::types::SpriteSizeType,
    dice_x: crate::types::SpriteSizeType,
    dice_y: crate::types::SpriteSizeType,
    filename: crate::types::FileName,
    layers: Vec<crate::types::Sprite>,
}

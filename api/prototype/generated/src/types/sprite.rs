#[derive(serde::Deserialize)]
pub struct Sprite {
    base_: crate::types::SpriteParameters,
    dice: Option<crate::types::SpriteSizeType>,
    dice_x: Option<crate::types::SpriteSizeType>,
    dice_y: Option<crate::types::SpriteSizeType>,
    filename: Option<crate::types::FileName>,
    layers: Option<Vec<crate::types::Sprite>>,
}

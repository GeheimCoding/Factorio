#[derive(serde::Deserialize)]
pub struct RotatedSpriteFrame {
    height: crate::types::SpriteSizeType,
    shift: crate::types::Vector,
    width: crate::types::SpriteSizeType,
    x: crate::types::SpriteSizeType,
    y: crate::types::SpriteSizeType,
}

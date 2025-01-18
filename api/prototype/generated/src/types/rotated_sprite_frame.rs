#[derive(serde::Deserialize)]
pub struct RotatedSpriteFrame {
    // default: inherited height
    height: crate::types::SpriteSizeType,
    // default: `{0, 0}`
    shift: crate::types::Vector,
    // default: inherited width
    width: crate::types::SpriteSizeType,
    #[serde(default = "default_x")]
    x: crate::types::SpriteSizeType,
    #[serde(default = "default_y")]
    y: crate::types::SpriteSizeType,
}
fn default_x() -> crate::types::SpriteSizeType {
    0
}
fn default_y() -> crate::types::SpriteSizeType {
    0
}

#[derive(serde::Deserialize)]
pub struct SpriteSource {
    filename: crate::types::FileName,
    height: crate::types::SpriteSizeType,
    #[serde(default = "default_load_in_minimal_mode")]
    load_in_minimal_mode: bool,
    position: (crate::types::SpriteSizeType, crate::types::SpriteSizeType),
    #[serde(default = "default_premul_alpha")]
    premul_alpha: bool,
    size: SpriteSourceSize,
    width: crate::types::SpriteSizeType,
    #[serde(default = "default_x")]
    x: crate::types::SpriteSizeType,
    #[serde(default = "default_y")]
    y: crate::types::SpriteSizeType,
}
fn default_load_in_minimal_mode() -> bool {
    false
}
fn default_premul_alpha() -> bool {
    true
}
#[derive(serde::Deserialize)]
pub enum SpriteSourceSize {
    #[serde(untagged)]
    SpriteSizeType(crate::types::SpriteSizeType),
    #[serde(untagged)]
    SpriteSizeTypeSpriteSizeType((crate::types::SpriteSizeType, crate::types::SpriteSizeType)),
}
fn default_x() -> crate::types::SpriteSizeType {
    0
}
fn default_y() -> crate::types::SpriteSizeType {
    0
}

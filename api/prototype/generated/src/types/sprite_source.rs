#[derive(Debug, serde::Deserialize)]
pub struct SpriteSource {
    filename: Option<crate::types::FileName>,
    height: Option<crate::types::SpriteSizeType>,
    #[serde(default = "default_load_in_minimal_mode")]
    load_in_minimal_mode: bool,
    position: Option<(crate::types::SpriteSizeType, crate::types::SpriteSizeType)>,
    #[serde(default = "default_premul_alpha")]
    premul_alpha: bool,
    size: Option<SpriteSourceSize>,
    width: Option<crate::types::SpriteSizeType>,
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
#[derive(Debug, serde::Deserialize)]
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

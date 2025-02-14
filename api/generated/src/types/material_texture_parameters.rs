#[derive(Debug, serde::Deserialize)]
pub struct MaterialTextureParameters {
    count: u32,
    #[serde(default = "default_line_length")]
    line_length: u32,
    picture: crate::types::FileName,
    #[serde(default = "default_scale")]
    scale: f32,
    #[serde(default = "default_x")]
    x: crate::types::SpriteSizeType,
    #[serde(default = "default_y")]
    y: crate::types::SpriteSizeType,
}
fn default_line_length() -> u32 {
    0
}
fn default_scale() -> f32 {
    1.0
}
fn default_x() -> crate::types::SpriteSizeType {
    0
}
fn default_y() -> crate::types::SpriteSizeType {
    0
}

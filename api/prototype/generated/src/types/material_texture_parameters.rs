#[derive(serde::Deserialize)]
pub struct MaterialTextureParameters {
    count: u32,
    line_length: u32,
    picture: crate::types::FileName,
    scale: f32,
    x: crate::types::SpriteSizeType,
    y: crate::types::SpriteSizeType,
}

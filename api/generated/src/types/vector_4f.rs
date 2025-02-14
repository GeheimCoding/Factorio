#[derive(Debug, serde::Deserialize)]
pub enum Vector4f {
    #[serde(untagged)]
    Vector4f { w: f32, x: f32, y: f32, z: f32 },
    #[serde(untagged)]
    F32F32F32F32((f32, f32, f32, f32)),
}

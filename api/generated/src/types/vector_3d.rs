#[derive(Debug, serde::Deserialize)]
pub enum Vector3D {
    #[serde(untagged)]
    Vector3D { x: f32, y: f32, z: f32 },
    #[serde(untagged)]
    F32F32F32((f32, f32, f32)),
}

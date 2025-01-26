#[derive(Debug, serde::Deserialize)]
pub enum RangedValue {
    #[serde(untagged)]
    F32F32((f32, f32)),
    #[serde(untagged)]
    F32(f32),
}

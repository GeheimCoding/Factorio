#[derive(serde::Deserialize)]
pub enum Color {
    #[serde(untagged)]
    Color { a: f32, b: f32, g: f32, r: f32 },
    #[serde(untagged)]
    F32F32F32((f32, f32, f32)),
    #[serde(untagged)]
    F32F32F32F32((f32, f32, f32, f32)),
}

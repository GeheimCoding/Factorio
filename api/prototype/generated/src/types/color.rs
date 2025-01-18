#[derive(serde::Deserialize)]
pub enum Color {
    #[serde(untagged)]
    Color {
        #[serde(default = "default_a")]
        a: f32,
        #[serde(default = "default_b")]
        b: f32,
        #[serde(default = "default_g")]
        g: f32,
        #[serde(default = "default_r")]
        r: f32,
    },
    #[serde(untagged)]
    F32F32F32((f32, f32, f32)),
    #[serde(untagged)]
    F32F32F32F32((f32, f32, f32, f32)),
}
fn default_a() -> f32 {
    1.0
}
fn default_b() -> f32 {
    0.0
}
fn default_g() -> f32 {
    0.0
}
fn default_r() -> f32 {
    0.0
}

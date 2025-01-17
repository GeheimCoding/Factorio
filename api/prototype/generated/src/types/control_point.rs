#[derive(serde::Deserialize)]
pub enum ControlPoint {
    #[serde(untagged)]
    ControlPoint {
        control: f32,
        volume_percentage: f32,
    },
    #[serde(untagged)]
    F32F32((f32, f32)),
}

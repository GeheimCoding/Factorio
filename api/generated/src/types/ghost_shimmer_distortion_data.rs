#[derive(Debug, serde::Deserialize)]
pub struct GhostShimmerDistortionData {
    intensity: f32,
    shape: i32,
    x: f32,
    y: f32,
}

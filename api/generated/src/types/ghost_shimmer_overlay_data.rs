#[derive(Debug, serde::Deserialize)]
pub struct GhostShimmerOverlayData {
    blend_mode: i32,
    cutoff: f32,
    shape: i32,
    tint: crate::types::Color,
    x: f32,
    y: f32,
}

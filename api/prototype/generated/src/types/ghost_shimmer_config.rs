#[derive(serde::Deserialize)]
pub struct GhostShimmerConfig {
    blend_mode: i32,
    distortion: f32,
    distortion_layers: Vec<crate::types::GhostShimmerDistortionData>,
    overlay_layers: Vec<crate::types::GhostShimmerOverlayData>,
    proportional_distortion: bool,
    tint: crate::types::Color,
    visualize_borders: bool,
    world_uv_modulo: i32,
}

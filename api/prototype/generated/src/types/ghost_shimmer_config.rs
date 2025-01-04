pub struct GhostShimmerConfig {
    blend_mode: i32,
    distortion: f32,
    distortion_layers: Vec<GhostShimmerDistortionData>,
    overlay_layers: Vec<GhostShimmerOverlayData>,
    proportional_distortion: bool,
    tint: Color,
    visualize_borders: bool,
    world_uv_modulo: i32,
}

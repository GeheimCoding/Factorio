#[derive(serde::Deserialize)]
pub struct RailFenceGraphicsSet {
    back_fence_render_layer: crate::types::RenderLayer,
    back_fence_render_layer_secondary: crate::types::RenderLayer,
    front_fence_render_layer: crate::types::RenderLayer,
    front_fence_render_layer_secondary: crate::types::RenderLayer,
    segment_count: u8,
    side_A: crate::types::RailFencePictureSet,
    side_B: crate::types::RailFencePictureSet,
}

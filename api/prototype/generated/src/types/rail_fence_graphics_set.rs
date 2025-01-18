#[derive(serde::Deserialize)]
pub struct RailFenceGraphicsSet {
    back_fence_render_layer: crate::types::RenderLayer,
    back_fence_render_layer_secondary: crate::types::RenderLayer,
    front_fence_render_layer: crate::types::RenderLayer,
    front_fence_render_layer_secondary: crate::types::RenderLayer,
    segment_count: u8,
    #[serde(rename = "side_A")]
    side_a: crate::types::RailFencePictureSet,
    #[serde(rename = "side_B")]
    side_b: crate::types::RailFencePictureSet,
}

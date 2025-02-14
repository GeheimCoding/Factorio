#[derive(Debug, serde::Deserialize)]
pub struct RailFenceGraphicsSet {
    #[serde(default = "default_back_fence_render_layer")]
    back_fence_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_back_fence_render_layer_secondary")]
    back_fence_render_layer_secondary: crate::types::RenderLayer,
    #[serde(default = "default_front_fence_render_layer")]
    front_fence_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_front_fence_render_layer_secondary")]
    front_fence_render_layer_secondary: crate::types::RenderLayer,
    segment_count: u8,
    #[serde(rename = "side_A")]
    side_a: crate::types::RailFencePictureSet,
    #[serde(rename = "side_B")]
    side_b: crate::types::RailFencePictureSet,
}
fn default_back_fence_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::ElevatedLowerObject
}
fn default_back_fence_render_layer_secondary() -> crate::types::RenderLayer {
    crate::types::RenderLayer::ElevatedLowerObject
}
fn default_front_fence_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::ElevatedHigherObject
}
fn default_front_fence_render_layer_secondary() -> crate::types::RenderLayer {
    crate::types::RenderLayer::ElevatedHigherObject
}

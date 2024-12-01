pub struct RailFenceGraphicsSet {
    back_fence_render_layer: RenderLayer,
    back_fence_render_layer_secondary: RenderLayer,
    front_fence_render_layer: RenderLayer,
    front_fence_render_layer_secondary: RenderLayer,
    segment_count: u8,
    side_A: RailFencePictureSet,
    side_B: RailFencePictureSet,
}

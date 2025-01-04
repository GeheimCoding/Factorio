pub struct GigaCargoHatchDefinition {
    closing_sound: crate::types::InterruptibleSound,
    covered_hatches: Vec<u32>,
    hatch_graphics_back: crate::types::Animation,
    hatch_graphics_front: crate::types::Animation,
    hatch_render_layer_back: crate::types::RenderLayer,
    hatch_render_layer_front: crate::types::RenderLayer,
    opening_sound: crate::types::InterruptibleSound,
}

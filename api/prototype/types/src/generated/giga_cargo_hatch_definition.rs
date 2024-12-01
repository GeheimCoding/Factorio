pub struct GigaCargoHatchDefinition {
    closing_sound: InterruptibleSound,
    covered_hatches: Vec<u32>,
    hatch_graphics_back: Animation,
    hatch_graphics_front: Animation,
    hatch_render_layer_back: RenderLayer,
    hatch_render_layer_front: RenderLayer,
    opening_sound: InterruptibleSound,
}

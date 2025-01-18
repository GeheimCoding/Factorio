#[derive(serde::Deserialize)]
pub struct GigaCargoHatchDefinition {
    closing_sound: crate::types::InterruptibleSound,
    covered_hatches: Vec<u32>,
    hatch_graphics_back: crate::types::Animation,
    hatch_graphics_front: crate::types::Animation,
    #[serde(default = "default_hatch_render_layer_back")]
    hatch_render_layer_back: crate::types::RenderLayer,
    #[serde(default = "default_hatch_render_layer_front")]
    hatch_render_layer_front: crate::types::RenderLayer,
    opening_sound: crate::types::InterruptibleSound,
}
fn default_hatch_render_layer_back() -> crate::types::RenderLayer {
    crate::types::RenderLayer::HigherObjectUnder
}
fn default_hatch_render_layer_front() -> crate::types::RenderLayer {
    crate::types::RenderLayer::HigherObjectAbove
}

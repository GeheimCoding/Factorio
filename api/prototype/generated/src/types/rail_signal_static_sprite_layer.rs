#[derive(serde::Deserialize)]
pub struct RailSignalStaticSpriteLayer {
    align_to_frame_index: Vec<u8>,
    #[serde(default = "default_hide_if_not_connected_to_rails")]
    hide_if_not_connected_to_rails: bool,
    #[serde(default = "default_hide_if_simulation")]
    hide_if_simulation: bool,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    shifts: Vec<crate::types::MapPosition>,
    sprites: crate::types::Animation,
}
fn default_hide_if_not_connected_to_rails() -> bool {
    true
}
fn default_hide_if_simulation() -> bool {
    true
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::RailChainSignalMetal
}

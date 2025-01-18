#[derive(serde::Deserialize)]
pub struct BeltReaderLayer {
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    sprites: crate::types::RotatedAnimation,
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::TransportBeltReader
}

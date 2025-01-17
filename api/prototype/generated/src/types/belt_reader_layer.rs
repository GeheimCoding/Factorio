#[derive(serde::Deserialize)]
pub struct BeltReaderLayer {
    render_layer: crate::types::RenderLayer,
    sprites: crate::types::RotatedAnimation,
}

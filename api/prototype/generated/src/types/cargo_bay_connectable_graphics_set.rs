#[derive(serde::Deserialize)]
pub struct CargoBayConnectableGraphicsSet {
    animation: crate::types::Animation,
    animation_render_layer: crate::types::RenderLayer,
    connections: crate::types::CargoBayConnections,
    picture: crate::types::LayeredSprite,
}

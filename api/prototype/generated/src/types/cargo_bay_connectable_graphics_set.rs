#[derive(serde::Deserialize)]
pub struct CargoBayConnectableGraphicsSet {
    animation: Option<crate::types::Animation>,
    // default: object
    animation_render_layer: Option<crate::types::RenderLayer>,
    connections: Option<crate::types::CargoBayConnections>,
    picture: Option<crate::types::LayeredSprite>,
}

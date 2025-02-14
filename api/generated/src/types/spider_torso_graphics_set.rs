#[derive(Debug, serde::Deserialize)]
pub struct SpiderTorsoGraphicsSet {
    animation: Option<crate::types::RotatedAnimation>,
    base_animation: Option<crate::types::RotatedAnimation>,
    #[serde(default = "default_base_render_layer")]
    base_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    shadow_animation: Option<crate::types::RotatedAnimation>,
    shadow_base_animation: Option<crate::types::RotatedAnimation>,
}
fn default_base_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::HigherObjectUnder
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::UnderElevated
}

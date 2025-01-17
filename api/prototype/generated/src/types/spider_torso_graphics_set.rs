#[derive(serde::Deserialize)]
pub struct SpiderTorsoGraphicsSet {
    animation: crate::types::RotatedAnimation,
    base_animation: crate::types::RotatedAnimation,
    base_render_layer: crate::types::RenderLayer,
    render_layer: crate::types::RenderLayer,
    shadow_animation: crate::types::RotatedAnimation,
    shadow_base_animation: crate::types::RotatedAnimation,
}

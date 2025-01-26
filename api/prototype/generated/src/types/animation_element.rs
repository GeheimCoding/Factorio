#[derive(Debug, serde::Deserialize)]
pub struct AnimationElement {
    #[serde(default = "default_always_draw")]
    always_draw: bool,
    animation: Option<crate::types::Animation>,
    #[serde(default = "default_apply_tint")]
    apply_tint: bool,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    secondary_draw_order: Option<i8>,
}
fn default_always_draw() -> bool {
    true
}
fn default_apply_tint() -> bool {
    false
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}

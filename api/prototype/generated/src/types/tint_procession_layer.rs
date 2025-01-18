#[derive(serde::Deserialize)]
pub struct TintProcessionLayer {
    frames: Vec<TintProcessionBezierControlPoint>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub struct TintProcessionBezierControlPoint {
    opacity: f64,
    opacity_t: f64,
    timestamp: crate::types::MapTick,
    tint_lower: crate::types::Color,
    tint_lower_t: crate::types::Color,
    tint_upper: crate::types::Color,
    tint_upper_t: crate::types::Color,
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}

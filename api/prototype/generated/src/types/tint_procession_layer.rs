#[derive(Debug, serde::Deserialize)]
pub struct TintProcessionLayer {
    frames: Vec<TintProcessionBezierControlPoint>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(Debug, serde::Deserialize)]
pub struct TintProcessionBezierControlPoint {
    opacity: Option<f64>,
    opacity_t: Option<f64>,
    timestamp: Option<crate::types::MapTick>,
    tint_lower: Option<crate::types::Color>,
    tint_lower_t: Option<crate::types::Color>,
    tint_upper: Option<crate::types::Color>,
    tint_upper_t: Option<crate::types::Color>,
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}

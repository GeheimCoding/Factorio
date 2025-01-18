#[derive(serde::Deserialize)]
pub struct TintProcessionLayer {
    frames: Vec<TintProcessionBezierControlPoint>,
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

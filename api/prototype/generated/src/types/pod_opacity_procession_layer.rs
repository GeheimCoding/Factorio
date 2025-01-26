#[derive(Debug, serde::Deserialize)]
pub struct PodOpacityProcessionLayer {
    frames: Vec<PodOpacityProcessionBezierControlPoint>,
    lut: crate::types::ColorLookupTable,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(Debug, serde::Deserialize)]
pub struct PodOpacityProcessionBezierControlPoint {
    cutscene_opacity: Option<f64>,
    cutscene_opacity_t: Option<f64>,
    lut_blend: Option<f64>,
    lut_blend_t: Option<f64>,
    outside_opacity: Option<f64>,
    outside_opacity_t: Option<f64>,
    timestamp: Option<crate::types::MapTick>,
}

#[derive(serde::Deserialize)]
pub struct PodOpacityProcessionLayer {
    frames: Vec<PodOpacityProcessionBezierControlPoint>,
    lut: crate::types::ColorLookupTable,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub struct PodOpacityProcessionBezierControlPoint {
    cutscene_opacity: f64,
    cutscene_opacity_t: f64,
    lut_blend: f64,
    lut_blend_t: f64,
    outside_opacity: f64,
    outside_opacity_t: f64,
    timestamp: crate::types::MapTick,
}

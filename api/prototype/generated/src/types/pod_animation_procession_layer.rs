#[derive(serde::Deserialize)]
pub struct PodAnimationProcessionLayer {
    frames: Vec<PodAnimationProcessionBezierControlPoint>,
    graphic: Option<crate::types::ProcessionGraphic>,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub struct PodAnimationProcessionBezierControlPoint {
    frame: f32,
    timestamp: crate::types::MapTick,
}

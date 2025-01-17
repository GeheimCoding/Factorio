#[derive(serde::Deserialize)]
pub struct PodAnimationProcessionLayer {
    frames: Vec<PodAnimationProcessionBezierControlPoint>,
    graphic: crate::types::ProcessionGraphic,
    type_: String,
}
#[derive(serde::Deserialize)]
pub struct PodAnimationProcessionBezierControlPoint {
    frame: f32,
    timestamp: crate::types::MapTick,
}

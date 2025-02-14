#[derive(Debug, serde::Deserialize)]
pub struct PodAnimationProcessionLayer {
    frames: crate::vec::Vec<PodAnimationProcessionBezierControlPoint>,
    graphic: Option<crate::types::ProcessionGraphic>,
}
#[derive(Debug, serde::Deserialize)]
pub struct PodAnimationProcessionBezierControlPoint {
    frame: f32,
    timestamp: crate::types::MapTick,
}

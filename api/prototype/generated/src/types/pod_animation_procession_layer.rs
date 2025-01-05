pub struct PodAnimationProcessionLayer {
    frames: Vec<PodAnimationProcessionBezierControlPoint>,
    graphic: crate::types::ProcessionGraphic,
    type_: String,
}
pub struct PodAnimationProcessionBezierControlPoint {
    frame: f32,
    timestamp: crate::types::MapTick,
}

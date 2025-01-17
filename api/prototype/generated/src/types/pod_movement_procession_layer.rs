#[derive(serde::Deserialize)]
pub struct PodMovementProcessionLayer {
    contribute_to_distance_traveled: bool,
    distance_traveled_contribution: f32,
    frames: Vec<PodMovementProcessionBezierControlPoint>,
    inherit_from: crate::types::ProcessionLayerInheritanceGroupID,
    reference_group: crate::types::ProcessionLayerInheritanceGroupID,
    type_: String,
}
#[derive(serde::Deserialize)]
pub struct PodMovementProcessionBezierControlPoint {
    offset: crate::types::Vector,
    offset_rate: f64,
    offset_rate_t: f64,
    offset_t: crate::types::Vector,
    tilt: f64,
    tilt_t: f64,
    timestamp: crate::types::MapTick,
}

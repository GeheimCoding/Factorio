#[derive(serde::Deserialize)]
pub struct PodMovementProcessionLayer {
    #[serde(default = "default_contribute_to_distance_traveled")]
    contribute_to_distance_traveled: bool,
    #[serde(default = "default_distance_traveled_contribution")]
    distance_traveled_contribution: f32,
    frames: Vec<PodMovementProcessionBezierControlPoint>,
    inherit_from: crate::types::ProcessionLayerInheritanceGroupID,
    reference_group: crate::types::ProcessionLayerInheritanceGroupID,
    #[serde(rename = "type")]
    type_: String,
}
fn default_contribute_to_distance_traveled() -> bool {
    true
}
fn default_distance_traveled_contribution() -> f32 {
    1.0
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

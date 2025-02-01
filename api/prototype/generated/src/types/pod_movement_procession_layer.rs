#[derive(Debug, serde::Deserialize)]
pub struct PodMovementProcessionLayer {
    #[serde(default = "default_contribute_to_distance_traveled")]
    contribute_to_distance_traveled: bool,
    #[serde(default = "default_distance_traveled_contribution")]
    distance_traveled_contribution: f32,
    frames: Vec<PodMovementProcessionBezierControlPoint>,
    inherit_from: Option<crate::types::ProcessionLayerInheritanceGroupID>,
    reference_group: Option<crate::types::ProcessionLayerInheritanceGroupID>,
}
fn default_contribute_to_distance_traveled() -> bool {
    true
}
fn default_distance_traveled_contribution() -> f32 {
    1.0
}
#[derive(Debug, serde::Deserialize)]
pub struct PodMovementProcessionBezierControlPoint {
    offset: Option<crate::types::Vector>,
    offset_rate: Option<f64>,
    offset_rate_t: Option<f64>,
    offset_t: Option<crate::types::Vector>,
    tilt: Option<f64>,
    tilt_t: Option<f64>,
    timestamp: Option<crate::types::MapTick>,
}

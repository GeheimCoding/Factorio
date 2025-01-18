#[derive(serde::Deserialize)]
pub struct PodDistanceTraveledProcessionLayer {
    contribute_to_distance_traveled: bool,
    distance_traveled_contribution: f32,
    frames: Vec<PodDistanceTraveledProcessionBezierControlPoint>,
    reference_group: crate::types::ProcessionLayerInheritanceGroupID,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub struct PodDistanceTraveledProcessionBezierControlPoint {
    distance: f64,
    distance_t: f64,
    timestamp: crate::types::MapTick,
}

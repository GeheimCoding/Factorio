#[derive(serde::Deserialize)]
pub struct PodDistanceTraveledProcessionLayer {
    #[serde(default = "default_contribute_to_distance_traveled")]
    contribute_to_distance_traveled: bool,
    #[serde(default = "default_distance_traveled_contribution")]
    distance_traveled_contribution: f32,
    frames: Vec<PodDistanceTraveledProcessionBezierControlPoint>,
    reference_group: Option<crate::types::ProcessionLayerInheritanceGroupID>,
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
pub struct PodDistanceTraveledProcessionBezierControlPoint {
    distance: Option<f64>,
    distance_t: Option<f64>,
    timestamp: Option<crate::types::MapTick>,
}

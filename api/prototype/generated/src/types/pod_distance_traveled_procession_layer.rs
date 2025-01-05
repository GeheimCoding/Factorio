pub struct PodDistanceTraveledProcessionLayer {
    contribute_to_distance_traveled: bool,
    distance_traveled_contribution: f32,
    frames: Vec<crate::types::PodDistanceTraveledProcessionBezierControlPoint>,
    reference_group: crate::types::ProcessionLayerInheritanceGroupID,
    type_: String,
}

pub struct PodDistanceTraveledProcessionLayer {
    contribute_to_distance_traveled: bool,
    distance_traveled_contribution: f32,
    frames: Vec<PodDistanceTraveledProcessionBezierControlPoint>,
    reference_group: ProcessionLayerInheritanceGroupID,
    type_: PodDistanceTraveled,
}

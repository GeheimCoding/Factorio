pub struct PodMovementProcessionLayer {
    contribute_to_distance_traveled: bool,
    distance_traveled_contribution: f32,
    frames: Vec<PodMovementProcessionBezierControlPoint>,
    inherit_from: ProcessionLayerInheritanceGroupID,
    reference_group: ProcessionLayerInheritanceGroupID,
    type_: PodMovement,
}

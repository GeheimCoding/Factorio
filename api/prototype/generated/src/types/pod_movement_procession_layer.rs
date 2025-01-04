pub struct PodMovementProcessionLayer {
    contribute_to_distance_traveled: bool,
    distance_traveled_contribution: f32,
    frames: Vec<crate::types::PodMovementProcessionBezierControlPoint>,
    inherit_from: crate::types::ProcessionLayerInheritanceGroupID,
    reference_group: crate::types::ProcessionLayerInheritanceGroupID,
    type_: PodMovement,
}

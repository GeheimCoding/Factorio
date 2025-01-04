pub struct TileBuildabilityRule {
    area: crate::types::SimpleBoundingBox,
    colliding_tiles: crate::types::CollisionMaskConnector,
    remove_on_collision: bool,
    required_tiles: crate::types::CollisionMaskConnector,
}

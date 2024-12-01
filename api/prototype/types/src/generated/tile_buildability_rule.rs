pub struct TileBuildabilityRule {
    area: SimpleBoundingBox,
    colliding_tiles: CollisionMaskConnector,
    remove_on_collision: bool,
    required_tiles: CollisionMaskConnector,
}

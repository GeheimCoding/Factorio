pub struct CollisionMaskConnector {
    colliding_with_tiles_only: bool,
    consider_tile_transitions: bool,
    layers: std::collections::HashMap<crate::types::CollisionLayerID, true>,
    not_colliding_with_itself: bool,
}

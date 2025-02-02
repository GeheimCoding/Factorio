#[derive(Debug, serde::Deserialize)]
pub struct CollisionMaskConnector {
    #[serde(default = "default_colliding_with_tiles_only")]
    colliding_with_tiles_only: bool,
    #[serde(default = "default_consider_tile_transitions")]
    consider_tile_transitions: bool,
    layers: std::collections::HashMap<crate::types::CollisionLayerID, bool>,
    #[serde(default = "default_not_colliding_with_itself")]
    not_colliding_with_itself: bool,
}
fn default_colliding_with_tiles_only() -> bool {
    false
}
fn default_consider_tile_transitions() -> bool {
    false
}
fn default_not_colliding_with_itself() -> bool {
    false
}

#[derive(serde::Deserialize)]
pub struct TileBuildabilityRule {
    area: crate::types::SimpleBoundingBox,
    // default: No masks
    colliding_tiles: Option<crate::types::CollisionMaskConnector>,
    #[serde(default = "default_remove_on_collision")]
    remove_on_collision: bool,
    // default: No masks
    required_tiles: Option<crate::types::CollisionMaskConnector>,
}
fn default_remove_on_collision() -> bool {
    false
}

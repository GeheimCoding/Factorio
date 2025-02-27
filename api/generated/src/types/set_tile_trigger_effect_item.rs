#[derive(Debug, serde::Deserialize)]
pub struct SetTileTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_apply_on_space_platform")]
    apply_on_space_platform: bool,
    #[serde(default = "default_apply_projection")]
    apply_projection: bool,
    radius: f32,
    tile_collision_mask: Option<crate::types::CollisionMaskConnector>,
    tile_name: crate::types::TileID,
}
fn default_apply_on_space_platform() -> bool {
    false
}
fn default_apply_projection() -> bool {
    false
}

#[derive(serde::Deserialize)]
pub struct SetTileTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    apply_on_space_platform: bool,
    apply_projection: bool,
    radius: f32,
    tile_collision_mask: crate::types::CollisionMaskConnector,
    tile_name: crate::types::TileID,
    type_: String,
}

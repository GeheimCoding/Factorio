#[derive(serde::Deserialize)]
pub struct InvokeTileEffectTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    tile_collision_mask: crate::types::CollisionMaskConnector,
    type_: String,
}

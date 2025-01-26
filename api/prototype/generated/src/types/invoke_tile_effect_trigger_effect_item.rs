#[derive(Debug, serde::Deserialize)]
pub struct InvokeTileEffectTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    tile_collision_mask: Option<crate::types::CollisionMaskConnector>,
    #[serde(rename = "type")]
    type_: String,
}

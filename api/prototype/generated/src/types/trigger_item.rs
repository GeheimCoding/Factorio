pub struct TriggerItem {
    action_delivery: TriggerItemActionDelivery,
    collision_mask: crate::types::CollisionMaskConnector,
    entity_flags: crate::types::EntityPrototypeFlags,
    force: crate::types::ForceCondition,
    ignore_collision_condition: bool,
    probability: f32,
    repeat_count: u32,
    trigger_target_mask: crate::types::TriggerTargetMask,
}
#[derive(serde::Deserialize)]
pub enum TriggerItemActionDelivery {
    #[serde(untagged)]
    TriggerDelivery(crate::types::TriggerDelivery),
    #[serde(untagged)]
    VecTriggerDelivery(Vec<crate::types::TriggerDelivery>),
}

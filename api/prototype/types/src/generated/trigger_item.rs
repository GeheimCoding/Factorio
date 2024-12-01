pub struct TriggerItem {
    action_delivery: TriggerItemActionDelivery,
    collision_mask: CollisionMaskConnector,
    entity_flags: EntityPrototypeFlags,
    force: ForceCondition,
    ignore_collision_condition: bool,
    probability: f32,
    repeat_count: u32,
    trigger_target_mask: TriggerTargetMask,
}
pub enum TriggerItemActionDelivery {
    TriggerDelivery(TriggerDelivery),
    VecTriggerDelivery(Vec<TriggerDelivery>),
}

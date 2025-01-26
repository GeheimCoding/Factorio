#[derive(Debug, serde::Deserialize)]
pub struct TriggerItem {
    action_delivery: Option<TriggerItemActionDelivery>,
    // default: All masks
    collision_mask: Option<crate::types::CollisionMaskConnector>,
    // default: All flags
    entity_flags: Option<crate::types::EntityPrototypeFlags>,
    // default: All forces
    force: Option<crate::types::ForceCondition>,
    #[serde(default = "default_ignore_collision_condition")]
    ignore_collision_condition: bool,
    #[serde(default = "default_probability")]
    probability: f32,
    #[serde(default = "default_repeat_count")]
    repeat_count: u32,
    // default: Everything
    trigger_target_mask: Option<crate::types::TriggerTargetMask>,
}
#[derive(Debug, serde::Deserialize)]
pub enum TriggerItemActionDelivery {
    #[serde(untagged)]
    TriggerDelivery(crate::types::TriggerDelivery),
    #[serde(untagged)]
    VecTriggerDelivery(Vec<crate::types::TriggerDelivery>),
}
fn default_ignore_collision_condition() -> bool {
    false
}
fn default_probability() -> f32 {
    1.0
}
fn default_repeat_count() -> u32 {
    1
}

#[derive(Debug, serde::Deserialize)]
pub struct DestroyCliffsTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    explosion_at_cliff: Option<crate::types::EntityID>,
    explosion_at_trigger: Option<crate::types::EntityID>,
    radius: f32,
}

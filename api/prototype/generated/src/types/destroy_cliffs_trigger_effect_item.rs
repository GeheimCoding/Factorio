#[derive(serde::Deserialize)]
pub struct DestroyCliffsTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    explosion_at_cliff: Option<crate::types::EntityID>,
    explosion_at_trigger: Option<crate::types::EntityID>,
    radius: f32,
    #[serde(rename = "type")]
    type_: String,
}

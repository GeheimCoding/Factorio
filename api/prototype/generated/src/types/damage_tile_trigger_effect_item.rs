#[derive(Debug, serde::Deserialize)]
pub struct DamageTileTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    damage: crate::types::DamageParameters,
    radius: Option<f32>,
}

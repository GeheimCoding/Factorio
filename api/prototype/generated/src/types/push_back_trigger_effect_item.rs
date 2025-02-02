#[derive(Debug, serde::Deserialize)]
pub struct PushBackTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    distance: f32,
}

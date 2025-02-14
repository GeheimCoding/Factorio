#[derive(Debug, serde::Deserialize)]
pub struct ScriptTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    effect_id: String,
}

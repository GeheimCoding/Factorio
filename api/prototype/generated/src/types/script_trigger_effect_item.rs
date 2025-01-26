#[derive(Debug, serde::Deserialize)]
pub struct ScriptTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    effect_id: String,
    #[serde(rename = "type")]
    type_: String,
}

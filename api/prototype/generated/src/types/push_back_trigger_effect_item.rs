#[derive(serde::Deserialize)]
pub struct PushBackTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    distance: f32,
    #[serde(rename = "type")]
    type_: String,
}

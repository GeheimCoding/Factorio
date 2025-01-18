#[derive(serde::Deserialize)]
pub struct ShowExplosionOnChartTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    scale: f32,
    #[serde(rename = "type")]
    type_: String,
}

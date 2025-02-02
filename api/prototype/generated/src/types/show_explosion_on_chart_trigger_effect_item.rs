#[derive(Debug, serde::Deserialize)]
pub struct ShowExplosionOnChartTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    scale: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct ShowExplosionOnChartTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    scale: f32,
}

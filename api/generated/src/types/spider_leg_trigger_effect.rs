#[derive(Debug, serde::Deserialize)]
pub struct SpiderLegTriggerEffect {
    effect: crate::types::TriggerEffect,
    position: f32,
}

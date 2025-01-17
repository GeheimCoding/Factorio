#[derive(serde::Deserialize)]
pub struct TimeSinceLastTipActivationTipTrigger {
    ticks: crate::types::MapTick,
    type_: String,
}

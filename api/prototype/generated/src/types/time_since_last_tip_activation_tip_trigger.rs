#[derive(serde::Deserialize)]
pub struct TimeSinceLastTipActivationTipTrigger {
    ticks: crate::types::MapTick,
    #[serde(rename = "type")]
    type_: String,
}

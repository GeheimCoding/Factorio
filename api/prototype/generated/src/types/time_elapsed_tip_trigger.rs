#[derive(Debug, serde::Deserialize)]
pub struct TimeElapsedTipTrigger {
    ticks: u32,
    #[serde(rename = "type")]
    type_: String,
}

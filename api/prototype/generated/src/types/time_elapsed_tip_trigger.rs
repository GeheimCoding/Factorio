#[derive(serde::Deserialize)]
pub struct TimeElapsedTipTrigger {
    ticks: u32,
    type_: String,
}

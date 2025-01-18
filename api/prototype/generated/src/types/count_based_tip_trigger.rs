#[derive(serde::Deserialize)]
pub struct CountBasedTipTrigger {
    #[serde(default = "default_count")]
    count: u32,
}
fn default_count() -> u32 {
    1
}

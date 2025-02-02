#[derive(Debug, serde::Deserialize)]
pub struct BeaconModuleVisualizations {
    art_style: String,
    slots: Option<crate::vec::Vec<crate::vec::Vec<crate::types::BeaconModuleVisualization>>>,
    #[serde(default = "default_tier_offset")]
    tier_offset: i32,
    #[serde(default = "default_use_for_empty_slots")]
    use_for_empty_slots: bool,
}
fn default_tier_offset() -> i32 {
    0
}
fn default_use_for_empty_slots() -> bool {
    false
}

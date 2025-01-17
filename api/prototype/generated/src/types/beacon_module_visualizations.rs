#[derive(serde::Deserialize)]
pub struct BeaconModuleVisualizations {
    art_style: String,
    slots: Vec<Vec<crate::types::BeaconModuleVisualization>>,
    tier_offset: i32,
    use_for_empty_slots: bool,
}

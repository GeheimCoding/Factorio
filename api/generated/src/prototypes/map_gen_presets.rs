#[derive(Debug, serde::Deserialize)]
pub struct MapGenPresets {
    name: String,
    #[serde(flatten)]
    custom_: std::collections::HashMap<String, crate::types::MapGenPreset>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MapGenPresets {
    name: String,
    custom_: std::collections::HashMap<String, crate::types::MapGenPreset>,
}

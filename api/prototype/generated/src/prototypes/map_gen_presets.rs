#[derive(serde::Deserialize)]
pub struct MapGenPresets {
    name: String,
    type_: String,
    custom_: std::collections::HashMap<String, crate::types::MapGenPreset>,
}

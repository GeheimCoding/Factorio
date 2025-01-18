#[derive(serde::Deserialize)]
pub struct MapGenPresets {
    name: String,
    #[serde(rename = "type")]
    type_: String,
    custom_: std::collections::HashMap<String, crate::types::MapGenPreset>,
}

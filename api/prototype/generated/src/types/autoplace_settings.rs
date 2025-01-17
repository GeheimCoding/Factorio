pub struct AutoplaceSettings {
    settings:
        std::collections::HashMap<AutoplaceSettingsSettings, crate::types::FrequencySizeRichness>,
    treat_missing_as_default: bool,
}
#[derive(serde::Deserialize)]
pub enum AutoplaceSettingsSettings {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    TileID(crate::types::TileID),
    #[serde(untagged)]
    DecorativeID(crate::types::DecorativeID),
}

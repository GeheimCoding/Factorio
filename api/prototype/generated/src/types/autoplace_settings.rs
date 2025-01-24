#[derive(serde::Deserialize)]
pub struct AutoplaceSettings {
    settings: Option<
        std::collections::HashMap<AutoplaceSettingsSettings, crate::types::FrequencySizeRichness>,
    >,
    treat_missing_as_default: Option<bool>,
}
#[derive(serde::Deserialize, PartialEq, Eq, Hash)]
pub enum AutoplaceSettingsSettings {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    TileID(crate::types::TileID),
    #[serde(untagged)]
    DecorativeID(crate::types::DecorativeID),
}

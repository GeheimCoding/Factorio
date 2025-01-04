pub struct AutoplaceSettings {
    settings:
        std::collections::HashMap<AutoplaceSettingsSettings, crate::types::FrequencySizeRichness>,
    treat_missing_as_default: bool,
}
pub enum AutoplaceSettingsSettings {
    EntityID(crate::types::EntityID),
    TileID(crate::types::TileID),
    DecorativeID(crate::types::DecorativeID),
}

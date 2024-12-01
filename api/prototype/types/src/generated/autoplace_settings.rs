pub struct AutoplaceSettings {
    settings: std::collections::HashMap<AutoplaceSettingsSettings, FrequencySizeRichness>,
    treat_missing_as_default: bool,
}
pub enum AutoplaceSettingsSettings {
    EntityID(EntityID),
    TileID(TileID),
    DecorativeID(DecorativeID),
}

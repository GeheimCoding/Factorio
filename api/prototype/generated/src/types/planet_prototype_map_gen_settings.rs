pub struct PlanetPrototypeMapGenSettings {
    autoplace_controls: std::collections::HashMap<AutoplaceControlID, FrequencySizeRichness>,
    autoplace_settings: std::collections::HashMap<
        PlanetPrototypeMapGenSettingsAutoplaceSettings,
        AutoplaceSettings,
    >,
    aux_climate_control: bool,
    cliff_settings: CliffPlacementSettings,
    moisture_climate_control: bool,
    property_expression_names:
        std::collections::HashMap<String, PlanetPrototypeMapGenSettingsPropertyExpressionNames>,
    territory_settings: TerritorySettings,
}
pub enum PlanetPrototypeMapGenSettingsAutoplaceSettings {
    Entity,
    Tile,
    Decorative,
}
pub enum PlanetPrototypeMapGenSettingsPropertyExpressionNames {
    String(String),
    Bool(bool),
    F64(f64),
}

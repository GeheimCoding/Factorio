pub struct PlanetPrototypeMapGenSettings {
    autoplace_controls: std::collections::HashMap<
        crate::types::AutoplaceControlID,
        crate::types::FrequencySizeRichness,
    >,
    autoplace_settings: std::collections::HashMap<
        PlanetPrototypeMapGenSettingsAutoplaceSettings,
        crate::types::AutoplaceSettings,
    >,
    aux_climate_control: bool,
    cliff_settings: crate::types::CliffPlacementSettings,
    moisture_climate_control: bool,
    property_expression_names:
        std::collections::HashMap<String, PlanetPrototypeMapGenSettingsPropertyExpressionNames>,
    territory_settings: crate::types::TerritorySettings,
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

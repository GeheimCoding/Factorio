#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
pub enum PlanetPrototypeMapGenSettingsAutoplaceSettings {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "tile")]
    Tile,
    #[serde(rename = "decorative")]
    Decorative,
}
#[derive(serde::Deserialize)]
pub enum PlanetPrototypeMapGenSettingsPropertyExpressionNames {
    #[serde(untagged)]
    String(String),
    #[serde(untagged)]
    Bool(bool),
    #[serde(untagged)]
    F64(f64),
}

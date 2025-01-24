#[derive(serde::Deserialize)]
pub struct PlanetPrototypeMapGenSettings {
    autoplace_controls: Option<
        std::collections::HashMap<
            crate::types::AutoplaceControlID,
            crate::types::FrequencySizeRichness,
        >,
    >,
    autoplace_settings: Option<
        std::collections::HashMap<
            PlanetPrototypeMapGenSettingsAutoplaceSettings,
            crate::types::AutoplaceSettings,
        >,
    >,
    #[serde(default = "default_aux_climate_control")]
    aux_climate_control: bool,
    cliff_settings: Option<crate::types::CliffPlacementSettings>,
    #[serde(default = "default_moisture_climate_control")]
    moisture_climate_control: bool,
    property_expression_names: Option<
        std::collections::HashMap<String, PlanetPrototypeMapGenSettingsPropertyExpressionNames>,
    >,
    territory_settings: Option<crate::types::TerritorySettings>,
}
#[derive(serde::Deserialize, PartialEq, Eq, Hash)]
pub enum PlanetPrototypeMapGenSettingsAutoplaceSettings {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "tile")]
    Tile,
    #[serde(rename = "decorative")]
    Decorative,
}
fn default_aux_climate_control() -> bool {
    false
}
fn default_moisture_climate_control() -> bool {
    false
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

#[derive(Debug, serde::Deserialize)]
pub struct MapGenSettings {
    autoplace_controls: Option<
        std::collections::HashMap<
            crate::types::AutoplaceControlID,
            crate::types::FrequencySizeRichness,
        >,
    >,
    autoplace_settings: Option<
        std::collections::HashMap<MapGenSettingsAutoplaceSettings, crate::types::AutoplaceSettings>,
    >,
    cliff_settings: Option<crate::types::CliffPlacementSettings>,
    #[serde(default = "default_default_enable_all_autoplace_controls")]
    default_enable_all_autoplace_controls: bool,
    height: Option<u32>,
    no_enemies_mode: Option<bool>,
    peaceful_mode: Option<bool>,
    property_expression_names:
        Option<std::collections::HashMap<String, MapGenSettingsPropertyExpressionNames>>,
    seed: Option<u32>,
    starting_area: Option<crate::types::MapGenSize>,
    starting_points: Option<Vec<crate::types::MapPosition>>,
    territory_settings: Option<crate::types::TerritorySettings>,
    width: Option<u32>,
}
#[derive(Debug, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum MapGenSettingsAutoplaceSettings {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "tile")]
    Tile,
    #[serde(rename = "decorative")]
    Decorative,
}
fn default_default_enable_all_autoplace_controls() -> bool {
    true
}
#[derive(Debug, serde::Deserialize)]
pub enum MapGenSettingsPropertyExpressionNames {
    #[serde(untagged)]
    String(String),
    #[serde(untagged)]
    Bool(bool),
    #[serde(untagged)]
    F64(f64),
}

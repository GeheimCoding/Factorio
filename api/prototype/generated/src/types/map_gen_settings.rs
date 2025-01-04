pub struct MapGenSettings {
    autoplace_controls: std::collections::HashMap<
        crate::types::AutoplaceControlID,
        crate::types::FrequencySizeRichness,
    >,
    autoplace_settings:
        std::collections::HashMap<MapGenSettingsAutoplaceSettings, crate::types::AutoplaceSettings>,
    cliff_settings: crate::types::CliffPlacementSettings,
    default_enable_all_autoplace_controls: bool,
    height: u32,
    no_enemies_mode: bool,
    peaceful_mode: bool,
    property_expression_names:
        std::collections::HashMap<String, MapGenSettingsPropertyExpressionNames>,
    seed: u32,
    starting_area: crate::types::MapGenSize,
    starting_points: Vec<crate::types::MapPosition>,
    territory_settings: crate::types::TerritorySettings,
    width: u32,
}
pub enum MapGenSettingsAutoplaceSettings {
    Entity,
    Tile,
    Decorative,
}
pub enum MapGenSettingsPropertyExpressionNames {
    String(String),
    Bool(bool),
    F64(f64),
}

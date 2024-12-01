pub struct MapGenSettings {
    autoplace_controls: std::collections::HashMap<AutoplaceControlID, FrequencySizeRichness>,
    autoplace_settings:
        std::collections::HashMap<MapGenSettingsAutoplaceSettings, AutoplaceSettings>,
    cliff_settings: CliffPlacementSettings,
    default_enable_all_autoplace_controls: bool,
    height: u32,
    no_enemies_mode: bool,
    peaceful_mode: bool,
    property_expression_names:
        std::collections::HashMap<String, MapGenSettingsPropertyExpressionNames>,
    seed: u32,
    starting_area: MapGenSize,
    starting_points: Vec<MapPosition>,
    territory_settings: TerritorySettings,
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

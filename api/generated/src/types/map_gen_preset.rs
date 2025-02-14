#[derive(Debug, serde::Deserialize)]
pub struct MapGenPreset {
    advanced_settings: Option<AdvancedMapGenSettings>,
    basic_settings: Option<crate::types::MapGenSettings>,
    #[serde(default = "default_default")]
    default: bool,
    order: crate::types::Order,
}
#[derive(Debug, serde::Deserialize)]
pub struct AdvancedMapGenSettings {
    asteroids: Option<crate::types::MapGenPresetAsteroidSettings>,
    difficulty_settings: Option<crate::types::MapGenPresetDifficultySettings>,
    enemy_evolution: Option<crate::types::MapGenPresetEnemyEvolutionSettings>,
    enemy_expansion: Option<crate::types::MapGenPresetEnemyExpansionSettings>,
    pollution: Option<crate::types::MapGenPresetPollutionSettings>,
}
fn default_default() -> bool {
    false
}

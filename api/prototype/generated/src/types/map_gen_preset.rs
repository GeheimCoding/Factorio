#[derive(serde::Deserialize)]
pub struct MapGenPreset {
    advanced_settings: AdvancedMapGenSettings,
    basic_settings: crate::types::MapGenSettings,
    #[serde(default = "default_default")]
    default: bool,
    order: crate::types::Order,
}
#[derive(serde::Deserialize)]
pub struct AdvancedMapGenSettings {
    asteroids: crate::types::MapGenPresetAsteroidSettings,
    difficulty_settings: crate::types::MapGenPresetDifficultySettings,
    enemy_evolution: crate::types::MapGenPresetEnemyEvolutionSettings,
    enemy_expansion: crate::types::MapGenPresetEnemyExpansionSettings,
    pollution: crate::types::MapGenPresetPollutionSettings,
}
fn default_default() -> bool {
    true
}

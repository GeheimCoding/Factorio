pub struct MapGenPreset {
    advanced_settings: AdvancedMapGenSettings,
    basic_settings: crate::types::MapGenSettings,
    default: bool,
    order: crate::types::Order,
}
pub struct AdvancedMapGenSettings {
    asteroids: crate::types::MapGenPresetAsteroidSettings,
    difficulty_settings: crate::types::MapGenPresetDifficultySettings,
    enemy_evolution: crate::types::MapGenPresetEnemyEvolutionSettings,
    enemy_expansion: crate::types::MapGenPresetEnemyExpansionSettings,
    pollution: crate::types::MapGenPresetPollutionSettings,
}

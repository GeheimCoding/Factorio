#[derive(Debug, serde::Deserialize)]
pub struct MapSettings {
    asteroids: crate::types::AsteroidSettings,
    difficulty_settings: crate::types::DifficultySettings,
    enemy_evolution: crate::types::EnemyEvolutionSettings,
    enemy_expansion: crate::types::EnemyExpansionSettings,
    max_failed_behavior_count: u32,
    name: String,
    path_finder: crate::types::PathFinderSettings,
    pollution: crate::types::PollutionSettings,
    steering: SteeringSettings,
    #[serde(rename = "type")]
    type_: String,
    unit_group: crate::types::UnitGroupSettings,
}
#[derive(Debug, serde::Deserialize)]
pub struct SteeringSettings {
    default: crate::types::StateSteeringSettings,
    moving: crate::types::StateSteeringSettings,
}

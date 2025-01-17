#[derive(serde::Deserialize)]
pub struct MapGenPresetEnemyEvolutionSettings {
    destroy_factor: f64,
    enabled: bool,
    pollution_factor: f64,
    time_factor: f64,
}

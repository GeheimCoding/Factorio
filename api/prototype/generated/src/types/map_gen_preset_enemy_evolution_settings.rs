#[derive(serde::Deserialize)]
pub struct MapGenPresetEnemyEvolutionSettings {
    destroy_factor: Option<f64>,
    enabled: Option<bool>,
    pollution_factor: Option<f64>,
    time_factor: Option<f64>,
}

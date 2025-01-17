#[derive(serde::Deserialize)]
pub struct EnemyEvolutionSettings {
    destroy_factor: f64,
    enabled: bool,
    pollution_factor: f64,
    time_factor: f64,
}

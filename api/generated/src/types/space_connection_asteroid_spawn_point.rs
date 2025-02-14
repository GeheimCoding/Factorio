#[derive(Debug, serde::Deserialize)]
pub struct SpaceConnectionAsteroidSpawnPoint {
    #[serde(flatten)]
    base_: crate::types::AsteroidSpawnPoint,
    distance: f64,
}

#[derive(serde::Deserialize)]
pub struct SpaceConnectionAsteroidSpawnPoint {
    base_: crate::types::AsteroidSpawnPoint,
    distance: f64,
}

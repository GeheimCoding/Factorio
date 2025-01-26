#[derive(Debug, serde::Deserialize)]
pub enum SpawnPoint {
    #[serde(untagged)]
    SpawnPoint {
        evolution_factor: f64,
        spawn_weight: f64,
    },
    #[serde(untagged)]
    F64F64((f64, f64)),
}

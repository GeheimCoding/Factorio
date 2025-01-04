pub enum SpawnPoint {
    SpawnPoint {
        evolution_factor: f64,
        spawn_weight: f64,
    },
    F64F64((f64, f64)),
}

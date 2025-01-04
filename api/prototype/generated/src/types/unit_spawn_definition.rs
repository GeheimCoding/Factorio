pub enum UnitSpawnDefinition {
    UnitSpawnDefinition {
        spawn_points: Vec<crate::types::SpawnPoint>,
        unit: crate::types::EntityID,
    },
    EntityIDVecSpawnPoint((crate::types::EntityID, Vec<crate::types::SpawnPoint>)),
}

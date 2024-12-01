pub enum UnitSpawnDefinition {
    UnitSpawnDefinition {
        spawn_points: Vec<SpawnPoint>,
        unit: EntityID,
    },
    EntityIDVecSpawnPoint((EntityID, Vec<SpawnPoint>)),
}

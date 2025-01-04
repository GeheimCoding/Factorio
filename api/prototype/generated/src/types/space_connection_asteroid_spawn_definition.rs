pub enum SpaceConnectionAsteroidSpawnDefinition {
    SpaceConnectionAsteroidSpawnDefinition {
        asteroid: SpaceConnectionAsteroidSpawnDefinitionAsteroid,
        spawn_points: Vec<crate::types::SpaceConnectionAsteroidSpawnPoint>,
        type_: SpaceConnectionAsteroidSpawnDefinitionType,
    },
    EntityIDVecSpaceConnectionAsteroidSpawnPoint(
        (
            crate::types::EntityID,
            Vec<crate::types::SpaceConnectionAsteroidSpawnPoint>,
        ),
    ),
}
pub enum SpaceConnectionAsteroidSpawnDefinitionAsteroid {
    EntityID(crate::types::EntityID),
    AsteroidChunkID(crate::types::AsteroidChunkID),
}
pub enum SpaceConnectionAsteroidSpawnDefinitionType {
    Entity,
    AsteroidChunk,
}

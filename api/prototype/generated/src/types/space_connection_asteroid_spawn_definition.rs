pub enum SpaceConnectionAsteroidSpawnDefinition {
    SpaceConnectionAsteroidSpawnDefinition {
        asteroid: SpaceConnectionAsteroidSpawnDefinitionAsteroid,
        spawn_points: Vec<SpaceConnectionAsteroidSpawnPoint>,
        type_: SpaceConnectionAsteroidSpawnDefinitionType,
    },
    EntityIDVecSpaceConnectionAsteroidSpawnPoint(
        (EntityID, Vec<SpaceConnectionAsteroidSpawnPoint>),
    ),
}
pub enum SpaceConnectionAsteroidSpawnDefinitionAsteroid {
    EntityID(EntityID),
    AsteroidChunkID(AsteroidChunkID),
}
pub enum SpaceConnectionAsteroidSpawnDefinitionType {
    Entity,
    AsteroidChunk,
}

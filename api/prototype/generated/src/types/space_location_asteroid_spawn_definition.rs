pub struct SpaceLocationAsteroidSpawnDefinition {
    asteroid: SpaceLocationAsteroidSpawnDefinitionAsteroid,
    type_: SpaceLocationAsteroidSpawnDefinitionType,
}
pub enum SpaceLocationAsteroidSpawnDefinitionAsteroid {
    EntityID(EntityID),
    AsteroidChunkID(AsteroidChunkID),
}
pub enum SpaceLocationAsteroidSpawnDefinitionType {
    Entity,
    AsteroidChunk,
}

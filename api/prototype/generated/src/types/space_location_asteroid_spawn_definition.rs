pub struct SpaceLocationAsteroidSpawnDefinition {
    asteroid: SpaceLocationAsteroidSpawnDefinitionAsteroid,
    type_: SpaceLocationAsteroidSpawnDefinitionType,
}
pub enum SpaceLocationAsteroidSpawnDefinitionAsteroid {
    EntityID(crate::types::EntityID),
    AsteroidChunkID(crate::types::AsteroidChunkID),
}
pub enum SpaceLocationAsteroidSpawnDefinitionType {
    Entity,
    AsteroidChunk,
}

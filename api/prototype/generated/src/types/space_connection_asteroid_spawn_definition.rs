#[derive(serde::Deserialize)]
pub enum SpaceConnectionAsteroidSpawnDefinition {
    #[serde(untagged)]
    SpaceConnectionAsteroidSpawnDefinition {
        asteroid: SpaceConnectionAsteroidSpawnDefinitionAsteroid,
        spawn_points: Vec<crate::types::SpaceConnectionAsteroidSpawnPoint>,
        #[serde(rename = "type")]
        type_: SpaceConnectionAsteroidSpawnDefinitionType,
    },
    #[serde(untagged)]
    EntityIDVecSpaceConnectionAsteroidSpawnPoint(
        (
            crate::types::EntityID,
            Vec<crate::types::SpaceConnectionAsteroidSpawnPoint>,
        ),
    ),
}
#[derive(serde::Deserialize)]
pub enum SpaceConnectionAsteroidSpawnDefinitionAsteroid {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    AsteroidChunkID(crate::types::AsteroidChunkID),
}
#[derive(serde::Deserialize)]
pub enum SpaceConnectionAsteroidSpawnDefinitionType {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "asteroid_chunk")]
    AsteroidChunk,
}

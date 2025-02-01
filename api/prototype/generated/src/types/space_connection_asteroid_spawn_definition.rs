#[derive(Debug, serde::Deserialize)]
pub enum SpaceConnectionAsteroidSpawnDefinition {
    #[serde(untagged)]
    SpaceConnectionAsteroidSpawnDefinition {
        asteroid: SpaceConnectionAsteroidSpawnDefinitionAsteroid,
        spawn_points: Vec<crate::types::SpaceConnectionAsteroidSpawnPoint>,
        #[serde(rename = "type")]
        #[serde(default = "default_type_")]
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
#[derive(Debug, serde::Deserialize)]
pub enum SpaceConnectionAsteroidSpawnDefinitionAsteroid {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    AsteroidChunkID(crate::types::AsteroidChunkID),
}
#[derive(Debug, serde::Deserialize)]
pub enum SpaceConnectionAsteroidSpawnDefinitionType {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "asteroid-chunk")]
    AsteroidChunk,
}
fn default_type_() -> SpaceConnectionAsteroidSpawnDefinitionType {
    SpaceConnectionAsteroidSpawnDefinitionType::Entity
}

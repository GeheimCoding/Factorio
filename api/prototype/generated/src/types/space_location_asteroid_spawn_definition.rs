#[derive(Debug, serde::Deserialize)]
pub struct SpaceLocationAsteroidSpawnDefinition {
    #[serde(flatten)]
    base_: crate::types::AsteroidSpawnPoint,
    asteroid: SpaceLocationAsteroidSpawnDefinitionAsteroid,
    #[serde(rename = "type")]
    #[serde(default = "default_type_")]
    type_: SpaceLocationAsteroidSpawnDefinitionType,
}
#[derive(Debug, serde::Deserialize)]
pub enum SpaceLocationAsteroidSpawnDefinitionAsteroid {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    AsteroidChunkID(crate::types::AsteroidChunkID),
}
#[derive(Debug, serde::Deserialize)]
pub enum SpaceLocationAsteroidSpawnDefinitionType {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "asteroid-chunk")]
    AsteroidChunk,
}
fn default_type_() -> SpaceLocationAsteroidSpawnDefinitionType {
    SpaceLocationAsteroidSpawnDefinitionType::Entity
}

#[derive(Debug, serde::Deserialize)]
pub enum UnitSpawnDefinition {
    #[serde(untagged)]
    UnitSpawnDefinition {
        spawn_points: Vec<crate::types::SpawnPoint>,
        unit: crate::types::EntityID,
    },
    #[serde(untagged)]
    EntityIDVecSpawnPoint((crate::types::EntityID, Vec<crate::types::SpawnPoint>)),
}

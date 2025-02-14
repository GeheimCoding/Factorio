#[derive(Debug, serde::Deserialize)]
pub enum UnitSpawnDefinition {
    #[serde(untagged)]
    UnitSpawnDefinition {
        spawn_points: crate::vec::Vec<crate::types::SpawnPoint>,
        unit: crate::types::EntityID,
    },
    #[serde(untagged)]
    EntityIDVecSpawnPoint(
        (
            crate::types::EntityID,
            crate::vec::Vec<crate::types::SpawnPoint>,
        ),
    ),
}

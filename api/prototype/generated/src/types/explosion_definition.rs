#[derive(serde::Deserialize)]
pub enum ExplosionDefinition {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    ExplosionDefinition {
        name: crate::types::EntityID,
        offset: crate::types::Vector,
    },
}

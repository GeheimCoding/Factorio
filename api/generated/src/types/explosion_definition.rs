#[derive(Debug, serde::Deserialize)]
pub enum ExplosionDefinition {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    ExplosionDefinition {
        name: crate::types::EntityID,
        offset: Option<Box<crate::types::Vector>>,
    },
}

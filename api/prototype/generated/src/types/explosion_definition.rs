pub enum ExplosionDefinition {
    EntityID(crate::types::EntityID),
    ExplosionDefinition {
        name: crate::types::EntityID,
        offset: crate::types::Vector,
    },
}

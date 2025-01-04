pub enum ExplosionDefinition {
    EntityID(EntityID),
    ExplosionDefinition { name: EntityID, offset: Vector },
}

pub enum EntityIDFilter {
    EntityIDFilter {
        comparator: ComparatorString,
        name: EntityID,
        quality: QualityID,
    },
    EntityID(EntityID),
}

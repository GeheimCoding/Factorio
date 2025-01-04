pub enum EntityIDFilter {
    EntityIDFilter {
        comparator: crate::types::ComparatorString,
        name: crate::types::EntityID,
        quality: crate::types::QualityID,
    },
    EntityID(crate::types::EntityID),
}

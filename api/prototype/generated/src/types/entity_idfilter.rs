#[derive(serde::Deserialize)]
pub enum EntityIDFilter {
    #[serde(untagged)]
    EntityIDFilter {
        comparator: crate::types::ComparatorString,
        name: crate::types::EntityID,
        quality: crate::types::QualityID,
    },
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
}

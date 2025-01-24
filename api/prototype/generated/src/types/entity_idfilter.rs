#[derive(serde::Deserialize)]
pub enum EntityIDFilter {
    #[serde(untagged)]
    EntityIDFilter {
        comparator: Option<crate::types::ComparatorString>,
        name: crate::types::EntityID,
        quality: Option<crate::types::QualityID>,
    },
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
}

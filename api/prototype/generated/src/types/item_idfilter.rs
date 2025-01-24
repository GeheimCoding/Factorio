#[derive(serde::Deserialize)]
pub enum ItemIDFilter {
    #[serde(untagged)]
    ItemIDFilter {
        comparator: Option<crate::types::ComparatorString>,
        name: crate::types::ItemID,
        quality: Option<crate::types::QualityID>,
    },
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
}

#[derive(serde::Deserialize)]
pub enum ItemIDFilter {
    #[serde(untagged)]
    ItemIDFilter {
        comparator: crate::types::ComparatorString,
        name: crate::types::ItemID,
        quality: crate::types::QualityID,
    },
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
}

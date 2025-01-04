pub enum ItemIDFilter {
    ItemIDFilter {
        comparator: crate::types::ComparatorString,
        name: crate::types::ItemID,
        quality: crate::types::QualityID,
    },
    ItemID(crate::types::ItemID),
}

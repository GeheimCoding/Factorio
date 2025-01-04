pub enum ItemIDFilter {
    ItemIDFilter {
        comparator: ComparatorString,
        name: ItemID,
        quality: QualityID,
    },
    ItemID(ItemID),
}

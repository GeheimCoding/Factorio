#[derive(Debug, serde::Deserialize)]
pub struct ItemToPlace {
    count: crate::types::ItemCountType,
    item: crate::types::ItemID,
}

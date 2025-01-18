#[derive(serde::Deserialize)]
pub struct CraftItemTechnologyTrigger {
    count: crate::types::ItemCountType,
    item: crate::types::ItemID,
    item_quality: crate::types::QualityID,
    #[serde(rename = "type")]
    type_: String,
}

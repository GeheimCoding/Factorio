#[derive(serde::Deserialize)]
pub struct CraftItemTechnologyTrigger {
    #[serde(default = "default_count")]
    count: crate::types::ItemCountType,
    item: crate::types::ItemID,
    item_quality: crate::types::QualityID,
    #[serde(rename = "type")]
    type_: String,
}
fn default_count() -> crate::types::ItemCountType {
    1
}

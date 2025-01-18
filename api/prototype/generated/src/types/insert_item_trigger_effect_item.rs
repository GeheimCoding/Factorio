#[derive(serde::Deserialize)]
pub struct InsertItemTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_count")]
    count: crate::types::ItemCountType,
    item: crate::types::ItemID,
    #[serde(default = "default_quality")]
    quality: crate::types::QualityID,
    #[serde(rename = "type")]
    type_: String,
}
fn default_count() -> crate::types::ItemCountType {
    1
}
fn default_quality() -> crate::types::QualityID {
    String::from("normal")
}

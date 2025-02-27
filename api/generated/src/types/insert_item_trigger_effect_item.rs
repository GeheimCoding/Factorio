#[derive(Debug, serde::Deserialize)]
pub struct InsertItemTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_count")]
    count: crate::types::ItemCountType,
    item: crate::types::ItemID,
    #[serde(default = "default_quality")]
    quality: crate::types::QualityID,
}
fn default_count() -> crate::types::ItemCountType {
    1
}
fn default_quality() -> crate::types::QualityID {
    String::from("normal")
}

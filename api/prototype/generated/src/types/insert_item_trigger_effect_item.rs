#[derive(serde::Deserialize)]
pub struct InsertItemTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    count: crate::types::ItemCountType,
    item: crate::types::ItemID,
    quality: crate::types::QualityID,
    #[serde(rename = "type")]
    type_: String,
}

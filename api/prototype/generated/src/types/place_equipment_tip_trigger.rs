#[derive(serde::Deserialize)]
pub struct PlaceEquipmentTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    equipment: crate::types::EquipmentID,
    #[serde(rename = "type")]
    type_: String,
}

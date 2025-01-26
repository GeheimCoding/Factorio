#[derive(Debug, serde::Deserialize)]
pub struct PlaceEquipmentTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    equipment: Option<crate::types::EquipmentID>,
    #[serde(rename = "type")]
    type_: String,
}

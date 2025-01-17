#[derive(serde::Deserialize)]
pub struct PlaceEquipmentTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    equipment: crate::types::EquipmentID,
    type_: String,
}

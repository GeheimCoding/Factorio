#[derive(serde::Deserialize)]
pub struct ActivateEquipmentCapsuleAction {
    equipment: crate::types::EquipmentID,
    type_: String,
}

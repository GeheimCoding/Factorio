#[derive(Debug, serde::Deserialize)]
pub struct ActivateEquipmentCapsuleAction {
    equipment: crate::types::EquipmentID,
}

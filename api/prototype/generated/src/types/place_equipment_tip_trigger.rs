#[derive(Debug, serde::Deserialize)]
pub struct PlaceEquipmentTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    equipment: Option<crate::types::EquipmentID>,
}

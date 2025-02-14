#[derive(Debug, serde::Deserialize)]
pub struct EquipmentShape {
    height: u32,
    points: Option<crate::vec::Vec<crate::vec::Vec<u32>>>,
    #[serde(rename = "type")]
    type_: EquipmentShapeType,
    width: u32,
}
#[derive(Debug, serde::Deserialize)]
pub enum EquipmentShapeType {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "manual")]
    Manual,
}

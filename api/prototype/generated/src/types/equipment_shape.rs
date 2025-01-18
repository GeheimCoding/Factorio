#[derive(serde::Deserialize)]
pub struct EquipmentShape {
    height: u32,
    points: Vec<Vec<u32>>,
    #[serde(rename = "type")]
    type_: EquipmentShapeType,
    width: u32,
}
#[derive(serde::Deserialize)]
pub enum EquipmentShapeType {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "manual")]
    Manual,
}

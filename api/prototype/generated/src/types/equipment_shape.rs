pub struct EquipmentShape {
    height: u32,
    points: Vec<Vec<u32>>,
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

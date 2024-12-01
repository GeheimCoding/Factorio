pub struct EquipmentShape {
    height: u32,
    points: Vec<Vec<u32>>,
    type_: EquipmentShapeType,
    width: u32,
}
pub enum EquipmentShapeType {
    Full,
    Manual,
}

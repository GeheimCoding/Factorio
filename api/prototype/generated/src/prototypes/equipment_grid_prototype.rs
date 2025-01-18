#[derive(serde::Deserialize)]
pub struct EquipmentGridPrototype {
    base_: crate::prototypes::Prototype,
    equipment_categories: Vec<crate::types::EquipmentCategoryID>,
    height: u32,
    #[serde(default = "default_locked")]
    locked: bool,
    width: u32,
}
fn default_locked() -> bool {
    false
}

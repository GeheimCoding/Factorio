#[derive(Debug, serde::Deserialize)]
pub struct EquipmentGhostPrototype {
    base_: crate::prototypes::EquipmentPrototype,
    categories: Option<Vec<crate::types::EquipmentCategoryID>>,
    energy_source: Option<crate::types::ElectricEnergySource>,
    shape: Option<crate::types::EquipmentShape>,
    take_result: Option<crate::types::ItemID>,
}

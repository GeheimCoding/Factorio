pub struct EquipmentGhostPrototype {
    base_: crate::prototypes::EquipmentPrototype,
    categories: Vec<crate::types::EquipmentCategoryID>,
    energy_source: crate::types::ElectricEnergySource,
    shape: crate::types::EquipmentShape,
    take_result: crate::types::ItemID,
}

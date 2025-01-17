#[derive(serde::Deserialize)]
pub struct EquipmentPrototype {
    base_: crate::prototypes::Prototype,
    background_border_color: crate::types::Color,
    background_color: crate::types::Color,
    categories: Vec<crate::types::EquipmentCategoryID>,
    energy_source: crate::types::ElectricEnergySource,
    grabbed_background_color: crate::types::Color,
    shape: crate::types::EquipmentShape,
    sprite: crate::types::Sprite,
    take_result: crate::types::ItemID,
}

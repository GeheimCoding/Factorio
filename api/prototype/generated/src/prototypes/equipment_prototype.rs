#[derive(Debug, serde::Deserialize)]
pub struct EquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    // default: equipment_default_background_border_color in utility constants
    background_border_color: Option<crate::types::Color>,
    // default: equipment_default_background_color in utility constants
    background_color: Option<crate::types::Color>,
    categories: Option<crate::vec::Vec<crate::types::EquipmentCategoryID>>,
    energy_source: Option<crate::types::ElectricEnergySource>,
    // default: equipment_default_grabbed_background_color in utility constants
    grabbed_background_color: Option<crate::types::Color>,
    shape: Option<crate::types::EquipmentShape>,
    sprite: crate::types::Sprite,
    // default: `name` of this prototype
    take_result: Option<crate::types::ItemID>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    // default: equipment_default_background_border_color in utility constants
    background_border_color: Option<crate::types::Color>,
    // default: equipment_default_background_color in utility constants
    background_color: Option<crate::types::Color>,
    // overridden by some child
    categories: Option<crate::vec::Vec<crate::types::EquipmentCategoryID>>,
    // overridden by some child
    energy_source: Option<crate::types::ElectricEnergySource>,
    // default: equipment_default_grabbed_background_color in utility constants
    grabbed_background_color: Option<crate::types::Color>,
    // overridden by some child
    shape: Option<crate::types::EquipmentShape>,
    sprite: crate::types::Sprite,
    // default: `name` of this prototype
    // overridden by some child
    take_result: Option<crate::types::ItemID>,
}

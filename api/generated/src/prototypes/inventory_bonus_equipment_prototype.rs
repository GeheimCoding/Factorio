#[derive(Debug, serde::Deserialize)]
pub struct InventoryBonusEquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EquipmentPrototype,
    energy_source: Option<crate::types::ElectricEnergySource>,
    inventory_size_bonus: crate::types::ItemStackIndex,
}

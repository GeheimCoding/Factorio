#[derive(Debug, serde::Deserialize)]
pub struct SolarPanelEquipmentPrototype {
    base_: crate::prototypes::EquipmentPrototype,
    power: crate::types::Energy,
}

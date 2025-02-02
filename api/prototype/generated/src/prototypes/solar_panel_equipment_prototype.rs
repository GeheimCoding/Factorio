#[derive(Debug, serde::Deserialize)]
pub struct SolarPanelEquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EquipmentPrototype,
    power: crate::types::Energy,
}

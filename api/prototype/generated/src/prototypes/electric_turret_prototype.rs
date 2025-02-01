#[derive(Debug, serde::Deserialize)]
pub struct ElectricTurretPrototype {
    base_: crate::prototypes::TurretPrototype,
    energy_source: ElectricTurretPrototypeEnergySource,
}
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ElectricTurretPrototypeEnergySource {
    #[serde(rename = "electric")]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(rename = "void")]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}

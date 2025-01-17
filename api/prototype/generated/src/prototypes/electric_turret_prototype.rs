pub struct ElectricTurretPrototype {
    base_: crate::prototypes::TurretPrototype,
    energy_source: ElectricTurretPrototypeEnergySource,
}
#[derive(serde::Deserialize)]
pub enum ElectricTurretPrototypeEnergySource {
    #[serde(untagged)]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}

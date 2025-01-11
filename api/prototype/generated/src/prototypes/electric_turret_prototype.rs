pub struct ElectricTurretPrototype {
    energy_source: ElectricTurretPrototypeEnergySource,
}
pub enum ElectricTurretPrototypeEnergySource {
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}

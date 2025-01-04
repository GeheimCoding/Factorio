pub enum EnergySource {
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    BurnerEnergySource(Box<crate::types::BurnerEnergySource>),
    HeatEnergySource(Box<crate::types::HeatEnergySource>),
    FluidEnergySource(Box<crate::types::FluidEnergySource>),
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}

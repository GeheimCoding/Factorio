pub enum EnergySource {
    ElectricEnergySource(Box<ElectricEnergySource>),
    BurnerEnergySource(Box<BurnerEnergySource>),
    HeatEnergySource(Box<HeatEnergySource>),
    FluidEnergySource(Box<FluidEnergySource>),
    VoidEnergySource(Box<VoidEnergySource>),
}

#[derive(Debug, serde::Deserialize)]
pub enum EnergySource {
    #[serde(untagged)]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(untagged)]
    BurnerEnergySource(Box<crate::types::BurnerEnergySource>),
    #[serde(untagged)]
    HeatEnergySource(Box<crate::types::HeatEnergySource>),
    #[serde(untagged)]
    FluidEnergySource(Box<crate::types::FluidEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}

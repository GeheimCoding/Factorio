#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum EnergySource {
    #[serde(rename = "electric")]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(rename = "burner")]
    BurnerEnergySource(Box<crate::types::BurnerEnergySource>),
    #[serde(rename = "heat")]
    HeatEnergySource(Box<crate::types::HeatEnergySource>),
    #[serde(rename = "fluid")]
    FluidEnergySource(Box<crate::types::FluidEnergySource>),
    #[serde(rename = "void")]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}

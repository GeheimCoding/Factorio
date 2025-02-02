#[derive(Debug, serde::Deserialize)]
pub struct FluidPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_auto_barrel")]
    auto_barrel: bool,
    base_color: crate::types::Color,
    default_temperature: f32,
    #[serde(default = "default_emissions_multiplier")]
    emissions_multiplier: f64,
    flow_color: crate::types::Color,
    #[serde(default = "default_fuel_value")]
    fuel_value: crate::types::Energy,
    // default: max value of float
    gas_temperature: Option<f32>,
    #[serde(default = "default_heat_capacity")]
    heat_capacity: crate::types::Energy,
    icon: Option<crate::types::FileName>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Option<crate::vec::Vec<crate::types::IconData>>,
    // default: value of `default_temperature`
    max_temperature: Option<f32>,
    visualization_color: Option<crate::types::Color>,
}
fn default_auto_barrel() -> bool {
    true
}
fn default_emissions_multiplier() -> f64 {
    1.0
}
fn default_fuel_value() -> crate::types::Energy {
    String::from("0J")
}
fn default_heat_capacity() -> crate::types::Energy {
    String::from("1kJ")
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}

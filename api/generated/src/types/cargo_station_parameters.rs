#[derive(Debug, serde::Deserialize)]
pub struct CargoStationParameters {
    giga_hatch_definitions: Option<crate::vec::Vec<crate::types::GigaCargoHatchDefinition>>,
    hatch_definitions: Option<crate::vec::Vec<crate::types::CargoHatchDefinition>>,
    #[serde(default = "default_prefer_packed_cargo_units")]
    prefer_packed_cargo_units: bool,
}
fn default_prefer_packed_cargo_units() -> bool {
    false
}

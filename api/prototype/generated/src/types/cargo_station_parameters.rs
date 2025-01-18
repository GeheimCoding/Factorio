#[derive(serde::Deserialize)]
pub struct CargoStationParameters {
    giga_hatch_definitions: Vec<crate::types::GigaCargoHatchDefinition>,
    hatch_definitions: Vec<crate::types::CargoHatchDefinition>,
    #[serde(default = "default_prefer_packed_cargo_units")]
    prefer_packed_cargo_units: bool,
}
fn default_prefer_packed_cargo_units() -> bool {
    false
}

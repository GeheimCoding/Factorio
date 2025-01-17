#[derive(serde::Deserialize)]
pub struct CargoStationParameters {
    giga_hatch_definitions: Vec<crate::types::GigaCargoHatchDefinition>,
    hatch_definitions: Vec<crate::types::CargoHatchDefinition>,
    prefer_packed_cargo_units: bool,
}

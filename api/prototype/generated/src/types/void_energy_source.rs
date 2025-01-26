#[derive(Debug, serde::Deserialize)]
pub struct VoidEnergySource {
    base_: crate::types::BaseEnergySource,
    #[serde(rename = "type")]
    type_: String,
}

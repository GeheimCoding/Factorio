#[derive(Debug, serde::Deserialize)]
pub struct VoidEnergySource {
    #[serde(flatten)]
    base_: crate::types::BaseEnergySource,
}

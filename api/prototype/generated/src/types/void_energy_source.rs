#[derive(serde::Deserialize)]
pub struct VoidEnergySource {
    base_: crate::types::BaseEnergySource,
    type_: String,
}

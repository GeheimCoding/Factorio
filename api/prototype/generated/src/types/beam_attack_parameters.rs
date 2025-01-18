#[derive(serde::Deserialize)]
pub struct BeamAttackParameters {
    base_: crate::types::BaseAttackParameters,
    #[serde(default = "default_source_direction_count")]
    source_direction_count: u32,
    source_offset: crate::types::Vector,
    #[serde(rename = "type")]
    type_: String,
}
fn default_source_direction_count() -> u32 {
    0
}

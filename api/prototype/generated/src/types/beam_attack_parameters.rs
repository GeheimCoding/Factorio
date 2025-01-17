#[derive(serde::Deserialize)]
pub struct BeamAttackParameters {
    base_: crate::types::BaseAttackParameters,
    source_direction_count: u32,
    source_offset: crate::types::Vector,
    type_: String,
}

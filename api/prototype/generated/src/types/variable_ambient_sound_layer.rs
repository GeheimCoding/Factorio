#[derive(serde::Deserialize)]
pub struct VariableAmbientSoundLayer {
    composition_mode: crate::types::VariableAmbientSoundCompositionMode,
    control_layer: String,
    control_layer_sample_mapping: Vec<Vec<u8>>,
    has_end_sample: bool,
    has_start_sample: bool,
    name: String,
    number_of_sublayers: u8,
    sample_length: crate::types::RandomRange,
    sublayer_offset: VariableAmbientSoundLayerSublayerOffset,
    sublayer_starting_offset: VariableAmbientSoundLayerSublayerStartingOffset,
    variants: Vec<crate::types::Sound>,
}
#[derive(serde::Deserialize)]
pub enum VariableAmbientSoundLayerSublayerOffset {
    #[serde(untagged)]
    RandomRange(crate::types::RandomRange),
    #[serde(untagged)]
    ProbabilityTable(crate::types::ProbabilityTable),
}
#[derive(serde::Deserialize)]
pub enum VariableAmbientSoundLayerSublayerStartingOffset {
    #[serde(untagged)]
    RandomRange(crate::types::RandomRange),
    #[serde(untagged)]
    ProbabilityTable(crate::types::ProbabilityTable),
}

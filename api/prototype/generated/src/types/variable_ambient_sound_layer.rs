#[derive(Debug, serde::Deserialize)]
pub struct VariableAmbientSoundLayer {
    composition_mode: crate::types::VariableAmbientSoundCompositionMode,
    control_layer: Option<String>,
    control_layer_sample_mapping: Option<crate::vec::Vec<crate::vec::Vec<u8>>>,
    #[serde(default = "default_has_end_sample")]
    has_end_sample: bool,
    #[serde(default = "default_has_start_sample")]
    has_start_sample: bool,
    name: String,
    #[serde(default = "default_number_of_sublayers")]
    number_of_sublayers: u8,
    sample_length: Option<crate::types::RandomRange>,
    sublayer_offset: Option<VariableAmbientSoundLayerSublayerOffset>,
    sublayer_starting_offset: Option<VariableAmbientSoundLayerSublayerStartingOffset>,
    variants: crate::vec::Vec<crate::types::Sound>,
}
fn default_has_end_sample() -> bool {
    false
}
fn default_has_start_sample() -> bool {
    false
}
fn default_number_of_sublayers() -> u8 {
    1
}
#[derive(Debug, serde::Deserialize)]
pub enum VariableAmbientSoundLayerSublayerOffset {
    #[serde(untagged)]
    RandomRange(crate::types::RandomRange),
    #[serde(untagged)]
    ProbabilityTable(crate::types::ProbabilityTable),
}
#[derive(Debug, serde::Deserialize)]
pub enum VariableAmbientSoundLayerSublayerStartingOffset {
    #[serde(untagged)]
    RandomRange(crate::types::RandomRange),
    #[serde(untagged)]
    ProbabilityTable(crate::types::ProbabilityTable),
}

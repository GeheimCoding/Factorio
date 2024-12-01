pub struct VariableAmbientSoundLayer {
    composition_mode: VariableAmbientSoundCompositionMode,
    control_layer: String,
    control_layer_sample_mapping: Vec<Vec<u8>>,
    has_end_sample: bool,
    has_start_sample: bool,
    name: String,
    number_of_sublayers: u8,
    sample_length: RandomRange,
    sublayer_offset: VariableAmbientSoundLayerSublayerOffset,
    sublayer_starting_offset: VariableAmbientSoundLayerSublayerStartingOffset,
    variants: Vec<Sound>,
}
pub enum VariableAmbientSoundLayerSublayerOffset {
    RandomRange(RandomRange),
    ProbabilityTable(ProbabilityTable),
}
pub enum VariableAmbientSoundLayerSublayerStartingOffset {
    RandomRange(RandomRange),
    ProbabilityTable(ProbabilityTable),
}

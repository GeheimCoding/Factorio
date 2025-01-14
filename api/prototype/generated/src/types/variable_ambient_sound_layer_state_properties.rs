pub struct VariableAmbientSoundLayerStateProperties {
    enabled: bool,
    end_pause: crate::types::RandomRange,
    number_of_repetitions: VariableAmbientSoundLayerStatePropertiesNumberOfRepetitions,
    pause_between_repetitions: crate::types::RandomRange,
    pause_between_samples: crate::types::RandomRange,
    sequence_length: crate::types::RandomRange,
    silence_instead_of_sample_probability: f32,
    start_pause: crate::types::RandomRange,
    variant: u8,
}
pub enum VariableAmbientSoundLayerStatePropertiesNumberOfRepetitions {
    RandomRange(crate::types::RandomRange),
    ProbabilityTable(crate::types::ProbabilityTable),
}

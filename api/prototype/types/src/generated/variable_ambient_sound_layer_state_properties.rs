pub struct VariableAmbientSoundLayerStateProperties {
    enabled: bool,
    end_pause: RandomRange,
    number_of_repetitions: VariableAmbientSoundLayerStatePropertiesNumberOfRepetitions,
    pause_between_repetitions: RandomRange,
    pause_between_samples: RandomRange,
    sequence_length: RandomRange,
    silence_instead_of_sample_probability: f32,
    start_pause: RandomRange,
    variant: u8,
}
pub enum VariableAmbientSoundLayerStatePropertiesNumberOfRepetitions {
    RandomRange(RandomRange),
    ProbabilityTable(ProbabilityTable),
}

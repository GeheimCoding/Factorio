#[derive(serde::Deserialize)]
pub struct VariableAmbientSoundLayerStateProperties {
    enabled: bool,
    end_pause: crate::types::RandomRange,
    number_of_repetitions: VariableAmbientSoundLayerStatePropertiesNumberOfRepetitions,
    pause_between_repetitions: crate::types::RandomRange,
    pause_between_samples: crate::types::RandomRange,
    sequence_length: crate::types::RandomRange,
    #[serde(default = "default_silence_instead_of_sample_probability")]
    silence_instead_of_sample_probability: f32,
    start_pause: crate::types::RandomRange,
    variant: u8,
}
#[derive(serde::Deserialize)]
pub enum VariableAmbientSoundLayerStatePropertiesNumberOfRepetitions {
    #[serde(untagged)]
    RandomRange(crate::types::RandomRange),
    #[serde(untagged)]
    ProbabilityTable(crate::types::ProbabilityTable),
}
fn default_silence_instead_of_sample_probability() -> f32 {
    0.0
}

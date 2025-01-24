#[derive(serde::Deserialize)]
pub struct VariableAmbientSoundLayerStateProperties {
    enabled: Option<bool>,
    end_pause: Option<crate::types::RandomRange>,
    number_of_repetitions: Option<VariableAmbientSoundLayerStatePropertiesNumberOfRepetitions>,
    pause_between_repetitions: Option<crate::types::RandomRange>,
    pause_between_samples: Option<crate::types::RandomRange>,
    sequence_length: Option<crate::types::RandomRange>,
    #[serde(default = "default_silence_instead_of_sample_probability")]
    silence_instead_of_sample_probability: f32,
    start_pause: Option<crate::types::RandomRange>,
    variant: Option<u8>,
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

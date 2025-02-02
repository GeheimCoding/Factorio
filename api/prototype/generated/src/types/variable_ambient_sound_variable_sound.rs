#[derive(Debug, serde::Deserialize)]
pub struct VariableAmbientSoundVariableSound {
    #[serde(default = "default_alignment_samples")]
    alignment_samples: u32,
    intermezzo: Option<crate::types::Sound>,
    layers: crate::vec::Vec<crate::types::VariableAmbientSoundLayer>,
    length_seconds: u32,
    states: crate::vec::Vec<crate::types::VariableAmbientSoundState>,
}
fn default_alignment_samples() -> u32 {
    12600
}

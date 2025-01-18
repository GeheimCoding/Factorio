#[derive(serde::Deserialize)]
pub struct VariableAmbientSoundVariableSound {
    #[serde(default = "default_alignment_samples")]
    alignment_samples: u32,
    intermezzo: crate::types::Sound,
    layers: Vec<crate::types::VariableAmbientSoundLayer>,
    length_seconds: u32,
    states: Vec<crate::types::VariableAmbientSoundState>,
}
fn default_alignment_samples() -> u32 {
    12600
}

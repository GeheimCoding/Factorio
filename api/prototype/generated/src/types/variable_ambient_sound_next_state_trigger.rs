#[derive(Debug, serde::Deserialize)]
pub enum VariableAmbientSoundNextStateTrigger {
    #[serde(rename = "layers_finished")]
    LayersFinished,
    #[serde(rename = "duration")]
    Duration,
}

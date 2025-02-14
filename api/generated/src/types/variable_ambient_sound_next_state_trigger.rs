#[derive(Debug, serde::Deserialize)]
pub enum VariableAmbientSoundNextStateTrigger {
    #[serde(rename = "layers-finished")]
    LayersFinished,
    #[serde(rename = "duration")]
    Duration,
}

#[derive(Debug, serde::Deserialize)]
pub enum VariableAmbientSoundStateType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "intermezzo")]
    Intermezzo,
    #[serde(rename = "final")]
    Final,
    #[serde(rename = "stop")]
    Stop,
}

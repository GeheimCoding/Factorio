#[derive(Debug, serde::Deserialize)]
pub struct ProcessionAudioEvent {
    audio: Option<crate::types::ProcessionAudio>,
    loop_id: Option<u32>,
    #[serde(rename = "type")]
    type_: crate::types::ProcessionAudioEventType,
    usage: Option<crate::types::ProcessionAudioUsage>,
}

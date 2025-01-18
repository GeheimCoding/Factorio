#[derive(serde::Deserialize)]
pub struct ProcessionAudioEvent {
    audio: crate::types::ProcessionAudio,
    loop_id: u32,
    #[serde(rename = "type")]
    type_: crate::types::ProcessionAudioEventType,
    usage: crate::types::ProcessionAudioUsage,
}
